use std::{
    path::Path,
    process::Child,
    sync::{Arc, Mutex},
};

use tracing::info;

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

    if let Some(mut child) = st.webui_child.take() {
        info!("Killing webui child process...");
        let _ = child.kill();
        let _ = child
            .wait()
            .expect("Failed to wait for webui child process");
        info!("Webui child process killed.");
    } else {
        info!("No webui child process to kill.");
    }

    if let Some(mut child) = st.playground_child.take() {
        info!("Killing playground child process...");
        let _ = child.kill();
        let _ = child
            .wait()
            .expect("Failed to wait for playground child process");
        info!("Playground child process killed.");
    } else {
        info!("No playground child process to kill.");
    }

    info!("Exiting...");
    std::process::exit(0);
}

pub fn run(cmd: DevCommand) -> anyhow::Result<()> {
    let root = workspace_root();
    let playground_server_name = "monoxide-playground";
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
            info!("Ctrl-C received, shutting down...");
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
        playground_server_name,
        webui_port,
        &playground_webui_dir,
    )?;
    {
        // graceful shutdown stuff
        let mut st = shutdown_state.lock().unwrap();
        st.playground_child = Some(playground_child);
    }

    // Exit if either server exits.
    loop {
        let mut st = shutdown_state.lock().unwrap();
        if let Some(ref mut child) = st.webui_child {
            if let Some(status) = child.try_wait().unwrap() {
                println!("Webui server exited with status {}. Exiting...", status);
                break;
            }
        }
        if let Some(ref mut child) = st.playground_child {
            if let Some(status) = child.try_wait().unwrap() {
                println!(
                    "Playground server exited with status {}. Exiting...",
                    status
                );
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

fn start_playground(
    cmd: &DevCommand,
    playground_server_name: &str,
    webui_port: Option<u16>,
    playground_webui_dir: &Path,
) -> anyhow::Result<Child> {
    // [cargo watch -i font --] \
    //   cargo run -p monoxide-playground -- \
    //   font \
    //   serve --port <port> [--reverse-proxy <url> | --serve-dir <dir>]
    let mut playground_cmd = std::process::Command::new(CARGO);
    if cmd.watch {
        playground_cmd.args(&["watch", "-i", "font", "--"]);
    }
    playground_cmd.args(&["run", "-p", playground_server_name, "--"]);
    // TODO: configurable font directory, currently hardcoded to `font`
    playground_cmd.args(&["font", "serve", "--port"]);
    playground_cmd.arg(cmd.port.to_string());
    if let Some(webui_port) = webui_port {
        playground_cmd.arg("--reverse-proxy");
        playground_cmd.arg(format!("http://127.0.0.1:{}", webui_port));
    } else {
        playground_cmd.arg("--serve-dir");
        playground_cmd.arg(playground_webui_dir.display().to_string());
    }
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
        let mut cmd = std::process::Command::new(pnpm);
        cmd.arg("dev")
            .arg("--host=127.0.0.1")
            .arg("--cors")
            .arg(format!("--port={}", (start_port + i as u16)))
            .arg("--strictPort")
            .current_dir(dir)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        let mut child = cmd.spawn().expect("Failed to run pnpm dev");
        // Wait a little while for the server to start
        std::thread::sleep(std::time::Duration::from_secs(1));
        // check if the server is running
        let status = child
            .try_wait()
            .expect("Failed to check if pnpm dev is running");

        if let Some(status) = status {
            println!(
                "Failed to start webui on port {}. Return status {}. Retrying...",
                start_port, status
            );
        }
    }

    Err(anyhow::anyhow!(
        "Failed to start webui after {} tries, including ports {} -- {}",
        RETRY_COUNT,
        start_port,
        start_port + RETRY_COUNT as u16 - 1
    ))
}
