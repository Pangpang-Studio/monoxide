use std::fmt::Write as _;

use monoxide_spiro::{BezCtx, SpiroCp};

#[derive(Debug, Default, Clone)]
struct TestBezCtx {
    buf: String,
}

impl TestBezCtx {
    const PRECISION: usize = 11;
}

impl BezCtx for TestBezCtx {
    fn move_to(&mut self, x: f64, y: f64, _is_open: bool) {
        let p = Self::PRECISION;
        _ = writeln!(self.buf, "M {x:.p$} {y:.p$}");
    }

    fn line_to(&mut self, x: f64, y: f64) {
        let p = Self::PRECISION;
        _ = writeln!(self.buf, "L {x:.p$} {y:.p$}");
    }

    fn quad_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p = Self::PRECISION;
        _ = writeln!(self.buf, "Q {x1:.p$} {y1:.p$}, {x2:.p$} {y2:.p$}");
    }

    fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let p = Self::PRECISION;
        _ = writeln!(
            self.buf,
            "C {x1:.p$} {y1:.p$}, {x2:.p$} {y2:.p$}, {x3:.p$} {y3:.p$}"
        );
    }

    fn mark_knot(&mut self, _knot_idx: usize) {
        _ = writeln!(self.buf);
    }
}

macro_rules! spiro_cp {
    ({ $x:literal, $y:literal, $ty:expr }) => {
        SpiroCp {
            x: $x as f64,
            y: $y as f64,
            ty: $ty,
        }
    };
}

#[test]
fn spiro_to_beziers() {
    use monoxide_spiro::SpiroCpTy::*;

    // Path from
    // https://github.com/fontforge/libspiro/blob/84ce4dfd24f0e3ee83589bbfb02723dff5c03414/tests/call-test.c#L278
    let path5 = [
        spiro_cp!({   0,    0, Open    }),
        spiro_cp!({ 100,  100, G2      }),
        spiro_cp!({ 200,  200, Left    }),
        spiro_cp!({ 300,  200, Right   }),
        spiro_cp!({ 400,  150, G2      }),
        spiro_cp!({ 300,  100, Left    }),
        spiro_cp!({ 200,  100, Right   }),
        spiro_cp!({ 150,   50, G2      }),
        spiro_cp!({ 100,    0, Left    }),
        spiro_cp!({   0, -100, Right   }),
        spiro_cp!({ -50, -200, G2      }),
        spiro_cp!({ -80, -250, EndOpen }),
    ];

    let mut ctx = TestBezCtx::default();
    assert!(ctx.run_spiro(&path5));

    // You may verify the output at <https://svg-path-visualizer.netlify.app>.
    insta::assert_snapshot!(&ctx.buf);
}
