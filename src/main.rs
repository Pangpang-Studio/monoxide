use std::fs::File;

use monoxide_font::make_font;
use monoxide_script::eval;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();
    eprintln!("Hello from Monoxide!");

    let res = eval::eval(
        &make_font(),
        &eval::AuxiliarySettings {
            point_per_em: 2048,
            font_name: "Monoxide".into(),
        },
    )?;

    let fout = "out.ttf";
    res.writer().autohint(true).write(File::create(fout)?)?;
    eprintln!("Successfully generated '{fout}'");

    Ok(())
}
