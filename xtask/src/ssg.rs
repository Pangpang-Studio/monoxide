use std::{path::PathBuf, process::Command};

use anyhow::bail;
use tracing::info;

use crate::{CARGO, util, workspace_root};

#[derive(clap::Parser)]
pub struct SsgCommand {
    /// Output directory for static web assets and generated font files.
    #[clap(default_value = "target/ssg")]
    out_dir: PathBuf,
}

pub fn run(cmd: SsgCommand) -> anyhow::Result<()> {
    let out_dir = workspace_root().join(cmd.out_dir);
    let assets_dir = out_dir.join("assets");

    let pnpm = util::resolve_pnpm();
    info!("Using pnpm = {pnpm}");

    info!("Building static web UI into {}", out_dir.display());
    util::build_playground_webui(&pnpm, Some(&out_dir), true)?;

    info!(
        "Rendering static font artifacts to {}",
        assets_dir.display()
    );

    let status = Command::new(CARGO)
        .args([
            "run",
            "-p",
            "monoxide-font",
            "--features",
            "playground",
            "--example",
            "playground",
            "--",
            "render",
        ])
        .arg(&assets_dir)
        .current_dir(workspace_root())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to render static font artifacts, status: {status}");
    }

    info!("Static playground generated in {}", out_dir.display());
    Ok(())
}
