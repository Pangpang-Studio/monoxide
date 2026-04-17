use std::fmt::Write as _;

use sirop::{Point, bez};

macro_rules! spiro_cp {
    ({ $x:literal, $y:literal, $ty:literal }) => {
        #[allow(clippy::cast_lossless, clippy::char_lit_as_u8, trivial_numeric_casts)]
        ::sirop::Cp {
            pt: ($x as f64, $y as f64),
            ty: ($ty as u8).try_into().unwrap(),
        }
    };
}

#[derive(Debug, Default, Clone)]
struct TestBezCtx {
    buf: String,
}

impl TestBezCtx {
    const PRECISION: usize = 8;
}

impl bez::Ctx for TestBezCtx {
    fn move_to(&mut self, (x, y): Point) {
        let p = Self::PRECISION;
        _ = writeln!(self.buf, "M {x:.p$} {y:.p$}");
    }

    fn line_to(&mut self, (x, y): Point) {
        let p = Self::PRECISION;
        _ = writeln!(self.buf, "L {x:.p$} {y:.p$}");
    }

    fn cubic_to(&mut self, (x1, y1): Point, (x2, y2): Point, (x3, y3): Point) {
        let p = Self::PRECISION;
        _ = writeln!(
            self.buf,
            "C {x1:.p$} {y1:.p$}, {x2:.p$} {y2:.p$}, {x3:.p$} {y3:.p$}"
        );
    }

    fn mark_knot(&mut self, _idx: usize) {
        _ = writeln!(self.buf);
    }
}

#[test]
fn bezier() -> sirop::Result<()> {
    let path5 = [
        spiro_cp!({  0,   0, '{'}),
        spiro_cp!({100, 100, 'c'}),
        spiro_cp!({200, 200, '['}),
        spiro_cp!({300, 200, ']'}),
        spiro_cp!({400, 150, 'c'}),
        spiro_cp!({300, 100, '['}),
        spiro_cp!({200, 100, ']'}),
        spiro_cp!({150,  50, 'c'}),
        spiro_cp!({100,   0, '['}),
        spiro_cp!({  0,-100, ']'}),
        spiro_cp!({-50,-200, 'c'}),
        spiro_cp!({-80,-250, '}'}),
    ];

    let ctx = &mut TestBezCtx::default();
    sirop::bezier(path5, ctx, false, None)?;

    // You may verify the output at <https://svg-path-visualizer.netlify.app>.
    insta::assert_snapshot!(&ctx.buf);

    Ok(())
}

#[test]
fn bezier_closed_flat_start_regression() -> sirop::Result<()> {
    // Closed contour whose first point is `Flat` (`'['`), which exercises the
    // cyclic solver index initialization path.
    let path = [
        spiro_cp!({100, 1800, '['}),
        spiro_cp!({100, 1200, ']'}),
        spiro_cp!({300,  900, 'o'}),
        spiro_cp!({500,  800, 'o'}),
        spiro_cp!({700,  900, 'o'}),
        spiro_cp!({900, 1200, '['}),
        spiro_cp!({900, 1800, ']'}),
        spiro_cp!({700, 2100, 'o'}),
        spiro_cp!({500, 2200, 'o'}),
        spiro_cp!({300, 2100, 'o'}),
    ];

    let ctx = &mut TestBezCtx::default();
    sirop::bezier(path, ctx, true, None)?;

    assert!(!ctx.buf.is_empty(), "bezier output should not be empty");
    let normalized = ctx.buf.to_ascii_lowercase();
    assert!(!normalized.contains("nan"), "output should not contain NaN");
    assert!(
        !normalized.contains("inf"),
        "output should not contain Infinity"
    );

    Ok(())
}

#[test]
fn bezier_closed_flat_start_from_stroke_regression() -> sirop::Result<()> {
    // Closed contour copied from glyph `o` stroke output where the first point
    // is `Flat` and previous sirop iterations produced non-finite controls.
    let path = [
        spiro_cp!({0.4375, 0.27890625, '['}),
        spiro_cp!({0.4375, 0.24609375, ']'}),
        spiro_cp!({0.409375, 0.08203125, 'o'}),
        spiro_cp!({0.25, -0.013125, 'o'}),
        spiro_cp!({0.090625, 0.08203125, 'o'}),
        spiro_cp!({0.0625, 0.24609375, '['}),
        spiro_cp!({0.0625, 0.27890625, ']'}),
        spiro_cp!({0.090625, 0.44296875, 'o'}),
        spiro_cp!({0.25, 0.538125, 'o'}),
        spiro_cp!({0.409375, 0.44296875, 'o'}),
    ];

    let ctx = &mut TestBezCtx::default();
    sirop::bezier(path, ctx, true, None)?;

    assert!(!ctx.buf.is_empty(), "bezier output should not be empty");
    let normalized = ctx.buf.to_ascii_lowercase();
    assert!(!normalized.contains("nan"), "output should not contain NaN");
    assert!(
        !normalized.contains("inf"),
        "output should not contain Infinity"
    );

    Ok(())
}
