mod svg;

use std::path::Path;

use itertools::Itertools;
use monoxide_script::ast::Glyph;
use snapbox::{Assert, Data, IntoData, assert};

use self::svg::{Scale, SvgPen, ViewBox};
use crate::{font::glyph, make_font_params};

fn snapshot_glyph_svg(name: &str, glyph: Glyph) -> assert::Result<()> {
    let mut pen = SvgPen::new(String::new(), Scale::default());
    pen.draw_glyph(&glyph).unwrap();
    let view_box = ViewBox::new(Scale::default());
    let svg_path = pen.finish();

    let svg_data = IntoData::into_data(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{view_box}">
  <path d="{}" fill="black" />
</svg>
"#,
        svg_path.trim()
    ));
    let file_data = Data::read_from(
        Path::new(&format!("./src/font/glyph/snapshots/{name}.svg")),
        None,
    );
    Assert::new()
        .action_env(assert::DEFAULT_ACTION_ENV)
        .try_eq(Some(&name), svg_data, file_data)
}

#[test]
fn snapshot_glyphs() {
    let cx = crate::InputContext {
        settings: make_font_params(),
    };
    let errs = glyph::GLYPH_FNS
        .iter()
        .filter_map(|&(ch, func)| {
            snapshot_glyph_svg(&format!("u{:0>6X}", ch as u32), func(&cx)).err()
        })
        .collect_vec();
    if errs.is_empty() {
        return;
    }
    panic!(
        "failed to snapshot {} glyphs:\n{}",
        errs.len(),
        errs.iter().join("\n")
    );
}
