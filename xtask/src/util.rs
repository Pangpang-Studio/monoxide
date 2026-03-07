use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

use crate::workspace_root;

pub fn resolve_pnpm() -> String {
    std::env::var("PNPM").unwrap_or_else(|_| if cfg!(windows) { "pnpm.cmd" } else { "pnpm" }.into())
}

pub fn playground_webui_dir() -> PathBuf {
    workspace_root().join("tools/playground")
}

pub fn playground_webui_dist_dir() -> PathBuf {
    playground_webui_dir().join("dist")
}

pub fn build_playground_webui(pnpm: &str, out_dir: Option<&Path>, static_mode: bool) -> Result<()> {
    let dir = playground_webui_dir();

    let mut cmd = Command::new(pnpm);
    cmd.arg("build");
    if let Some(out_dir) = out_dir {
        cmd.arg("--outDir").arg(out_dir);
    }
    if static_mode {
        cmd.env("VITE_PLAYGROUND_DEPLOY_MODE", "static");
    }
    cmd.current_dir(&dir);

    let status = cmd.status().with_context(|| {
        format!(
            "Failed to run `{} build` in {}",
            pnpm,
            dir.to_string_lossy()
        )
    })?;

    if !status.success() {
        bail!("Failed to build playground webui, status: {status}");
    }

    Ok(())
}
