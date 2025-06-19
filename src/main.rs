use monoxide_font::make_font;
use monoxide_script::eval::AuxiliarySettings;

fn main() {
    eprintln!("Hello from Monoxide!");
    let fcx = make_font().expect("Unable to create font");
    let res = monoxide_script::eval::eval(
        &fcx,
        &AuxiliarySettings {
            point_per_em: 2048,
            font_name: "Monoxide".into(),
        },
    );
    let fout = "out.ttf";
    res.write(std::fs::File::create(fout).expect("Failed to open file"))
        .expect("Failed to write font");
    eprintln!("Successfully generated '{fout}'");
}
