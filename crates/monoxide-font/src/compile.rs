use monoxide_script::eval::AuxiliarySettings;

mod font;

fn main() {
    let fcx = font::make_font().expect("Unable to create font");
    let res = monoxide_script::eval::eval(
        &fcx,
        &AuxiliarySettings {
            point_per_em: 2048,
            font_name: "Monoxide".into(),
        },
    );
    res.write(std::fs::File::create("out.ttf").expect("Failed to open file"))
        .expect("Failed to write font");
}
