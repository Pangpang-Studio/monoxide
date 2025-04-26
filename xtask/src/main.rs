mod dev;

use clap::Parser;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CARGO: &str = env!("CARGO");

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    let cmd = Command::parse();
    match cmd {
        Command::Dev(dev_command) => dev::run(dev_command).expect("Failed to run dev command"),
    }
}

/// Get the root of the workspace. Since we live in `$/xtask`, it's simply the
/// parent of the manifest directory.
fn workspace_root() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(MANIFEST_DIR);
    path.pop(); // Remove the `xtask` directory
    path
}

#[derive(clap::Parser)]
enum Command {
    Dev(dev::DevCommand),
}
