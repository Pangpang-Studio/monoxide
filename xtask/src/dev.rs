use std::{
    borrow::Cow,
    io,
    net::{SocketAddrV4, TcpStream},
    path::Path,
    process::Child,
    sync::{Arc, Mutex},
};

use tracing::{info, warn};

use crate::{CARGO, workspace_root};

#[derive(clap::Parser)]
pub struct DevCommand {
    /// The port to run the server on. Defaults to 3070.
    ///
    /// Note that we will _not_ try to find a spare port if this is taken.
    #[clap(short, long, default_value = "3070")]
    port: u16,

    /// Develop the web UI too. If unset, will use an existing build of the
    /// playground web UI. Will try to build it if it doesn't exist.
    #[clap(long, alias = "playground")]
    also_webui: bool,

    /// Force a rebuild of the playground web UI.
    #[clap(long, alias = "build-playground")]
    build_webui: bool,

    /// The port to run the webui dev server on. Only used if `--also-webui` is
    /// set. Will try to find a spare port if taken.
    #[clap(short, long, default_value = "5173")]
    webui_port: u16,

    /// Watch the playground project for changes and rebuild it automatically.
    #[clap(long, default_value = "false")]
    watch: bool,
}

#[derive(Debug, Default)]
struct ShutdownState {
    webui_child: Option<Child>,
    playground_child: Option<Child>,
}

fn graceful_shutdown(st: &Mutex<ShutdownState>) {
    let mut st = st
        .lock()
        .unwrap_or_else(|_| panic!("Failed to lock mutex! Please manually kill the process."));

    tracing::error!("Performing graceful shutdown...");

    if let Some(mut child) = st.webui_child.take() {
        info!("Killing webui child process...");
        gracefully_kill(&mut child).expect("Failed to kill webui child process");
        info!("Webui child process killed.");
    } else {
        info!("No webui child process to kill.");
    }

    if let Some(mut child) = st.playground_child.take() {
        info!("Killing playground child process...");
        _ = gracefully_kill(&mut child).inspect_err(|e| warn!("{e:?}"));
        info!("Playground child process killed.");
    } else {
        info!("No playground child process to kill.");
    }

    info!("Until next time!");
    std::process::exit(0);
}

pub fn run(cmd: DevCommand) -> anyhow::Result<()> {
    let root = workspace_root();
    let playground_crate = "monoxide-font";
    let playground_example = "playground";
    let playground_webui_dir = root.join("tools/playground");
    let playground_webui_dist = playground_webui_dir.join("dist");

    // We use PNPM to develop everything
    let pnpm = std::env::var("PNPM").unwrap_or_else(|_| {
        if cfg!(windows) {
            "pnpm.cmd".to_string()
        } else {
            "pnpm".to_string()
        }
    });
    info!("Using pnpm = {}", pnpm);

    // Graceful shutdown stuff
    let shutdown_state = Arc::new(Mutex::new(ShutdownState::default()));
    {
        let st = shutdown_state.clone();
        ctrlc::set_handler(move || {
            tracing::warn!("Ctrl-C received, shutting down...");
            graceful_shutdown(&st);
        })
        .expect("Failed to set Ctrl-C handler");
    }
    scopeguard::defer!({
        info!("Cleaning up...");
        graceful_shutdown(&shutdown_state);
    });

    // All set. Start the server(s).

    // First, start or build the webui server.
    let webui_built = playground_webui_dist.exists();
    let (webui_port, webui_child) = if cmd.also_webui {
        info!("Starting webui dev server on port {}", cmd.webui_port);
        let (port, child) = start_dev_webui(&pnpm, &playground_webui_dir, cmd.webui_port)?;

        info!("Webui dev server started on port {}", port);
        Some((port, child))
    } else {
        if !webui_built || cmd.build_webui {
            info!("Building webui...");
            build_webui(&pnpm, &playground_webui_dir);
        }
        info!("Built webui found at {}", playground_webui_dist.display());
        None
    }
    .unzip();
    {
        // graceful shutdown stuff
        let mut st = shutdown_state.lock().unwrap();
        st.webui_child = webui_child;
    }

    // Then start the playground server.
    let playground_child = start_playground(
        &cmd,
        playground_crate,
        playground_example,
        webui_port,
        &playground_webui_dist,
    )?;
    {
        // graceful shutdown stuff
        let mut st = shutdown_state.lock().unwrap();
        st.playground_child = Some(playground_child);
    }

    // Wait for the servers to come online
    loop {
        if check_exit_status(&shutdown_state) {
            anyhow::bail!("One of the servers exited. Exiting...");
        }
        let online = is_port_online(cmd.port).unwrap_or(false);
        if online {
            break;
        } else {
            info!("Waiting for playground server on port {}...", cmd.port);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    info!("All servers started successfully!");
    info!("");
    warn!(
        "Open the Playground WebUI at http://127.0.0.1:{}/",
        cmd.port
    );
    warn!("");
    warn!("Press Ctrl-C to exit.");

    // Exit if either server exits.
    loop {
        if check_exit_status(&shutdown_state) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

/// Check if the webui or playground server has exited. If so, return true
fn check_exit_status(shutdown_state: &Arc<Mutex<ShutdownState>>) -> bool {
    let mut st = shutdown_state.lock().unwrap();
    if let Some(ref mut child) = st.webui_child {
        if let Some(status) = child.try_wait().unwrap() {
            tracing::error!("Webui server exited with status {}. Exiting...", status);
            return true;
        }
    }
    if let Some(ref mut child) = st.playground_child {
        if let Some(status) = child.try_wait().unwrap() {
            tracing::error!(
                "Playground server exited with status {}. Exiting...",
                status
            );
            return true;
        }
    }
    false
}

fn start_playground(
    cmd: &DevCommand,
    playground_crate: &str,
    playground_example: &str,
    webui_port: Option<u16>,
    playground_webui_dir: &Path,
) -> anyhow::Result<Child> {
    // [cargo watch -i font --] \
    //   dx serve --hotpatch -p monoxide-playground -- \
    //   serve --port=<port> [--reverse-proxy=<url> | --serve-dir=<dir>]
    let mut playground_cmd;
    if cmd.watch {
        playground_cmd = std::process::Command::new(CARGO);
        playground_cmd.args(["watch", "-i", "xtask", "-i", "tools", "--", "dx"]);
    } else {
        playground_cmd = std::process::Command::new("dx");
    }
    playground_cmd.args([
        "serve",
        "--hotpatch",
        "-p",
        playground_crate,
        "--example",
        playground_example,
        "--features=playground",
    ]);
    let mut playground_args = vec![Cow::from("serve"), format!("--port={}", cmd.port).into()];
    if let Some(webui_port) = webui_port {
        playground_args.push(format!("--reverse-proxy=http://127.0.0.1:{webui_port}").into());
    } else {
        playground_args.push(format!("--serve-dir={}", playground_webui_dir.display()).into());
    }
    playground_cmd.arg("--args").arg(playground_args.join(" "));
    playground_cmd.stdout(std::process::Stdio::inherit());
    playground_cmd.stderr(std::process::Stdio::inherit());
    playground_cmd.current_dir(workspace_root());
    info!("Starting playground server...");
    let mut child = playground_cmd
        .spawn()
        .expect("Failed to run playground server");

    // Wait a little while for the server to start
    std::thread::sleep(std::time::Duration::from_secs(1));
    // check if the server is running
    let status = child
        .try_wait()
        .expect("Failed to check if playground server is running");

    if let Some(status) = status {
        println!(
            "Failed to start playground server on port {}. Return status {}.",
            cmd.port, status
        );
        panic!("Failed to start playground server");
    }
    info!("Playground server started on port {}", cmd.port);
    Ok(child)
}

fn build_webui(pnpm: &str, dir: &Path) {
    let mut cmd = std::process::Command::new(pnpm);
    cmd.arg("build");
    cmd.current_dir(dir);
    let status = cmd.status().expect("Failed to run pnpm build");
    if !status.success() {
        panic!("Failed to build playground, status: {}", status);
    }
}

const RETRY_COUNT: usize = 10;

/// Start the webui dev server. This will return the port it is running on.
/// If start of webui fails, it will retry to start it on a different port.
fn start_dev_webui(pnpm: &str, dir: &Path, start_port: u16) -> anyhow::Result<(u16, Child)> {
    for i in 0..RETRY_COUNT {
        let port = start_port + i as u16;
        let mut cmd = std::process::Command::new(pnpm);
        cmd.arg("dev")
            .arg("--host=127.0.0.1")
            .arg("--cors")
            .arg(format!("--port={}", port))
            .arg("--strictPort")
            .arg("--clearScreen=false")
            .current_dir(dir)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        let mut child = cmd.spawn().expect("Failed to run pnpm dev");

        // Give the server a little time to start
        let start_timeout = 10;

        let mut is_online = false;
        for _ in 0..start_timeout {
            std::thread::sleep(std::time::Duration::from_secs(1));
            // check if the server is running
            let status = child
                .try_wait()
                .expect("Failed to check if pnpm dev is running");

            if let Some(status) = status {
                warn!(
                    "Failed to start webui on port {}. Return status {}. Retrying...",
                    start_port, status
                );
                is_online = false;
                break;
            }

            let online = is_port_online(port).unwrap_or(false);
            if online {
                is_online = true;
                break;
            } else {
                info!("Webui not online yet on port {}. Retrying...", port);
            }
        }

        if is_online {
            info!("Webui dev server started on port {}", port);
            return Ok((port, child));
        } else {
            // If the server is not online, kill the child process
            // and try the next port
            let _ = child.kill();
            let _ = child.wait().expect("Failed to wait for pnpm dev process");
            warn!(
                "Webui server not online after {} seconds. Retrying on port {}...",
                start_timeout, port
            );
            continue;
        }
    }

    Err(anyhow::anyhow!(
        "Failed to start webui after {} tries, including ports {} -- {}",
        RETRY_COUNT,
        start_port,
        start_port + RETRY_COUNT as u16 - 1
    ))
}

/// Check if the given port is online by attempting a single connection.
/// Returns Ok(true) if connection succeeds, Ok(false) if it fails with
/// ConnectionRefused or TimedOut, and Err for other IO errors.
fn is_port_online(port: u16) -> anyhow::Result<bool> {
    let addr = SocketAddrV4::new([127, 0, 0, 1].into(), port);
    // Use a short timeout to avoid blocking for long
    match TcpStream::connect_timeout(&addr.into(), std::time::Duration::from_millis(200)) {
        Ok(_) => Ok(true), // Connection successful
        Err(e)
            if e.kind() == io::ErrorKind::ConnectionRefused
                || e.kind() == io::ErrorKind::TimedOut =>
        {
            Ok(false) // Port not listening or connection timed out
        }
        Err(e) => {
            // Other unexpected error
            warn!("Unexpected error checking port {}: {}", port, e);
            Err(e.into())
        }
    }
}

/// Gracefully kill a whole process tree, and wait for it to exit.
fn gracefully_kill(child: &mut Child) -> anyhow::Result<()> {
    #[cfg(unix)]
    unsafe {
        let success = libc::kill(child.id() as i32, libc::SIGTERM);
        if success != 0 {
            return Err(anyhow::anyhow!(
                "Failed to kill process: {}",
                io::Error::last_os_error()
            ));
        }
        child.wait()?;
        Ok(())
    }
    #[cfg(windows)]
    {
        child.kill()?;
        child.wait()?;
        Ok(())
    }
}
