mod svg;

use std::path::Path;

use itertools::Itertools;
use monoxide_script::ast::Glyph;
use snapbox::{Assert, Data, IntoData as _, assert};

use self::svg::{Scale, SvgPen, ViewBox};
use crate::{glyph, make_font_params};

fn snapshot_glyph_svg(name: &str, glyph: Glyph) -> assert::Result<()> {
    let mut pen = SvgPen::new(String::new(), Scale::default());
    pen.draw_glyph(&glyph).unwrap();
    let view_box = ViewBox::new(Scale::default());
    let svg_path = pen.finish();

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{view_box}">
  <path d="{}" fill="black" />
</svg>
"#,
        svg_path.trim()
    );

    let snapshot_data = Data::read_from(
        Path::new(&format!("./src/glyph/snapshots/{name}.svg")),
        None,
    );

    // NOTE: Tiny floating-point drifts (e.g. FMA contraction on different CPUs)
    // may slightly modify the resulting SVG, so we only fall back to a
    // literal comparison (which also handles snapshot refreshing) when the
    // SVGs are not considered equivalent.
    if let Some(snapshot) = snapshot_data.render()
        && svg::equiv(&svg, &snapshot)
    {
        return Ok(());
    }

    Assert::new().action_env(assert::DEFAULT_ACTION_ENV).try_eq(
        Some(&name),
        svg.into_data(),
        snapshot_data,
    )
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
