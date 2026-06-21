use monoxide_font::make_font;
use monoxide_script::eval::AuxiliarySettings;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();
    eprintln!("Hello from Monoxide!");

    let res = monoxide_script::eval::eval(
        &make_font(),
        &AuxiliarySettings {
            point_per_em: 2048,
            font_name: "Monoxide".into(),
        },
    )
    .expect("Failed to evaluate font context");
    let fout = "out.ttf";
    res.write(std::fs::File::create(fout).expect("Failed to open file"))
        .expect("Failed to write font");
    eprintln!("Successfully generated '{fout}'");
}
