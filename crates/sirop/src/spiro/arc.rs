use super::K as SpiroK;
use crate::{
    Point,
    bez::{self, Ctx, Strand},
};

/// An [`Arc`] is defined as:
///
/// $$
/// \int_{-\frac{1}{2}}^{\frac{1}{2}} \exp\left(i \left(k\_0 s + \frac{ k\_1
/// s^2}{2} + \frac{k\_2 s^3}{6} + \frac{k\_3 s^4}{24}\right)\right) ds
/// $$
#[cfg_attr(doc, katexit::katexit)]
#[derive(Debug, Clone, Copy)]
pub struct Arc {
    pub p0: Point,
    pub p1: Point,

    rot: f64,
    ks: SpiroK,

    pub derive_x0: f64,
    pub derive_y0: f64,
    pub derive_x1: f64,
    pub derive_y1: f64,

    pub len: f64,
    pub bend: f64,
}

impl Arc {
    #[allow(clippy::similar_names)]
    #[must_use]
    pub fn new(ks: SpiroK, p0: Point, p1: Point) -> Self {
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        let seg_ch = (x1 - x0).hypot(y1 - y0);
        let seg_th = (y1 - y0).atan2(x1 - x0);

        let (sx, sy) = ks.integrate_spiro();
        let ch = sx.hypot(sy);
        let th = sy.atan2(sx);
        let arc_len = seg_ch / ch;

        let rot = seg_th - th;
        let theta_left = rot + ks.theta(-0.5, -0.5);
        let theta_right = rot + ks.theta(0.5, 0.5);

        Self {
            p0,
            p1,
            ks,
            rot,
            derive_x0: arc_len * theta_left.cos(),
            derive_y0: arc_len * theta_left.sin(),
            derive_x1: arc_len * theta_right.cos(),
            derive_y1: arc_len * theta_right.sin(),
            len: seg_ch / ch,
            bend: ks.bend(),
        }
    }

    #[must_use]
    pub fn to_cubic(&self) -> bez::Strand {
        bez::Strand::Cubic {
            start: self.p0,
            c1: (
                self.p0.0 + self.derive_x0 / 3.,
                self.p0.1 + self.derive_y0 / 3.,
            ),
            c2: (
                self.p1.0 - self.derive_x1 / 3.,
                self.p1.1 - self.derive_y1 / 3.,
            ),
            end: self.p1,
        }
    }

    #[must_use]
    pub fn eval(&self, at: f64) -> Point {
        let t = at - 0.5;
        self.eval_k_sub(t, &self.ks.divide(-0.5, t))
    }

    fn eval_k_sub(&self, t: f64, k_sub: &SpiroK) -> Point {
        let th_sub = self.rot + self.ks.theta(-0.5, t);
        let cth = (t + 0.5) * self.len * th_sub.cos();
        let sth = (t + 0.5) * self.len * th_sub.sin();
        let (sx, sy) = k_sub.integrate_spiro();
        let x_mid = self.p0.0 + cth * sx - sth * sy;
        let y_mid = self.p0.1 + cth * sy + sth * sx;
        (x_mid, y_mid)
    }

    #[must_use]
    pub fn derivative(&self, at: f64) -> Point {
        let t = at - 0.5;
        let theta = self.rot + self.ks.theta(t, t);
        (self.len * theta.cos(), self.len * theta.sin())
    }

    #[must_use]
    pub fn subdivide(&self, at: f64) -> (Self, Self) {
        let t = at - 0.5;
        let k_sub = self.ks.divide(-0.5, t);
        let k_sub_rear = self.ks.divide(t, 0.5);

        let mid = self.eval_k_sub(t, &k_sub);
        (
            Self::new(k_sub, self.p0, mid),
            Self::new(k_sub_rear, mid, self.p1),
        )
    }

    #[must_use]
    pub(crate) fn subdivisions(&self, delta: f64) -> Vec<Self> {
        fn go(this: Arc, delta: f64, depth: usize, sink: &mut Vec<Arc>) {
            if depth > 5 || this.bend <= delta {
                sink.push(this);
                return;
            }
            let (head, tail) = this.subdivide(0.5);
            go(head, delta, depth + 1, sink);
            go(tail, delta, depth + 1, sink);
        }

        let mut sink = Vec::with_capacity(32);
        go(*self, delta, 0, &mut sink);
        sink
    }

    pub(crate) fn render_bez<C: Ctx>(&self, ctx: &mut C, delta: f64) {
        const ARC_STRAIGHT_EPSILON: f64 = 1e-8;
        if self.bend <= ARC_STRAIGHT_EPSILON {
            ctx.line_to(self.p1);
            return;
        }
        for part in self.subdivisions(delta) {
            let Strand::Cubic { c1, c2, end, .. } = part.to_cubic() else {
                unreachable!();
            };
            ctx.cubic_to(c1, c2, end);
        }
    }
}
