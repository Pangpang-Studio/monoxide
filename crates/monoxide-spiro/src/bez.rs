use crate::{SpiroCp, SpiroCpTy};

pub trait BezCtx {
    fn move_to(&mut self, x: f64, y: f64, is_open: bool);
    fn line_to(&mut self, x: f64, y: f64);
    fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    fn mark_knot(&mut self, knot_idx: usize);

    fn start(&mut self) {}
    fn end(&mut self) {}

    #[must_use]
    fn run_spiro(&mut self, ps: &[SpiroCp]) -> bool
    where
        Self: Sized,
    {
        let is_closed = match ps {
            [] => true, // Empty contour
            [SpiroCp { ty, .. }, ..] => !matches!(ty, SpiroCpTy::Open),
        };

        let mut ctx = BezCtxAdapter {
            ctx: self,
            is_closed,
        };
        let ps = ps.iter().copied().map(sirop::Cp::from);
        sirop::bezier(ps, &mut ctx, is_closed, None).is_ok()
    }
}

struct BezCtxAdapter<'a, T> {
    ctx: &'a mut T,
    is_closed: bool,
}

impl<T: BezCtx> sirop::bez::Ctx for BezCtxAdapter<'_, T> {
    fn move_to(&mut self, to: sirop::Point) {
        self.ctx.move_to(to.0, to.1, !self.is_closed);
    }

    fn line_to(&mut self, to: sirop::Point) {
        self.ctx.line_to(to.0, to.1);
    }

    fn cubic_to(&mut self, c1: sirop::Point, c2: sirop::Point, to: sirop::Point) {
        self.ctx.curve_to(c1.0, c1.1, c2.0, c2.1, to.0, to.1);
    }

    fn mark_knot(&mut self, idx: usize) {
        self.ctx.mark_knot(idx);
    }

    fn start(&mut self) {
        self.ctx.start();
    }

    fn end(&mut self) {
        self.ctx.end();
    }
}
