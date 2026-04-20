use crate::Point;

#[derive(Default, Debug, Clone, Copy)]
pub struct K(pub [f64; 4]);

impl K {
    /// Integrate polynomial spiral curve from -0.5 to 0.5.
    #[allow(clippy::many_single_char_names)]
    #[must_use]
    pub fn integrate_spiro(&self) -> Point {
        let [k0, k1, k2, k3] = self.0;
        let iters = 4;

        let th1 = k0;
        let th2 = k1 / 2.;
        let th3 = k2 / 6.;
        let th4 = k3 / 24.;
        let ds = f64::from(iters).recip();
        let ds2 = ds * ds;
        let ds3 = ds2 * ds;
        let k0 = k0 * ds;
        let k1 = k1 * ds;
        let k2 = k2 * ds;
        let k3 = k3 * ds;

        let mut s = ds.mul_add(0.5, -0.5);

        let mut x = 0.;
        let mut y = 0.;
        for _ in 0..iters {
            let km0 = (k3 / 6.).mul_add(s, k2 / 2.).mul_add(s, k1).mul_add(s, k0);
            let km1 = (0.5 * k3).mul_add(s, k2).mul_add(s, k1) * ds;
            let km2 = k3.mul_add(s, k2) * ds2;
            let km3 = k3 * ds3;

            // Order 12 implementation
            let t1_1 = km0;
            let t1_2 = km1 / 2.;
            let t1_3 = km2 / 6.;
            let t1_4 = km3 / 24.;
            let t2_2 = t1_1 * t1_1;
            let t2_3 = 2. * (t1_1 * t1_2);
            let t2_4 = (t1_1 * t1_3).mul_add(2., t1_2 * t1_2);
            let t2_5 = 2. * (t1_1 * t1_4 + t1_2 * t1_3);
            let t2_6 = (t1_2 * t1_4).mul_add(2., t1_3 * t1_3);
            let t2_7 = 2. * (t1_3 * t1_4);
            let t2_8 = t1_4 * t1_4;
            let t3_4 = t2_2 * t1_2 + t2_3 * t1_1;
            let t3_6 = t2_2 * t1_4 + t2_3 * t1_3 + t2_4 * t1_2 + t2_5 * t1_1;
            let t3_8 = t2_4 * t1_4 + t2_5 * t1_3 + t2_6 * t1_2 + t2_7 * t1_1;
            let t3_10 = t2_6 * t1_4 + t2_7 * t1_3 + t2_8 * t1_2;
            let t4_4 = t2_2 * t2_2;
            let t4_5 = 2. * (t2_2 * t2_3);
            let t4_6 = (t2_2 * t2_4).mul_add(2., t2_3 * t2_3);
            let t4_7 = 2. * (t2_2 * t2_5 + t2_3 * t2_4);
            let t4_8 = (t2_2 * t2_6 + t2_3 * t2_5).mul_add(2., t2_4 * t2_4);
            let t4_9 = 2. * (t2_2 * t2_7 + t2_3 * t2_6 + t2_4 * t2_5);
            let t4_10 = (t2_2 * t2_8 + t2_3 * t2_7 + t2_4 * t2_6).mul_add(2., t2_5 * t2_5);
            let t5_6 = t4_4 * t1_2 + t4_5 * t1_1;
            let t5_8 = t4_4 * t1_4 + t4_5 * t1_3 + t4_6 * t1_2 + t4_7 * t1_1;
            let t5_10 = t4_6 * t1_4 + t4_7 * t1_3 + t4_8 * t1_2 + t4_9 * t1_1;
            let t6_6 = t4_4 * t2_2;
            let t6_7 = t4_4 * t2_3 + t4_5 * t2_2;
            let t6_8 = t4_4 * t2_4 + t4_5 * t2_3 + t4_6 * t2_2;
            let t6_9 = t4_4 * t2_5 + t4_5 * t2_4 + t4_6 * t2_3 + t4_7 * t2_2;
            let t6_10 = t4_4 * t2_6 + t4_5 * t2_5 + t4_6 * t2_4 + t4_7 * t2_3 + t4_8 * t2_2;
            let t7_8 = t6_6 * t1_2 + t6_7 * t1_1;
            let t7_10 = t6_6 * t1_4 + t6_7 * t1_3 + t6_8 * t1_2 + t6_9 * t1_1;
            let t8_8 = t6_6 * t2_2;
            let t8_9 = t6_6 * t2_3 + t6_7 * t2_2;
            let t8_10 = t6_6 * t2_4 + t6_7 * t2_3 + t6_8 * t2_2;
            let t9_10 = t8_8 * t1_2 + t8_9 * t1_1;
            let t10_10 = t8_8 * t2_2;

            let mut u = 1.;
            let mut v = 0.;

            v += t1_2.mul_add(1. / 12., t1_4 / 80.);
            u -= t2_2.mul_add(
                1. / 24.,
                t2_4.mul_add(1. / 160., t2_6.mul_add(1. / 896., t2_8 / 4608.)),
            );

            v -= t3_4.mul_add(
                1. / 480.,
                t3_6.mul_add(1. / 2688., t3_8.mul_add(1. / 13824., t3_10 / 67584.)),
            );
            u += t4_4.mul_add(
                1. / 1920.,
                t4_6.mul_add(1. / 10752., t4_8.mul_add(1. / 55296., t4_10 / 270_336.)),
            );

            v += t5_6.mul_add(
                1. / 53760.,
                t5_8.mul_add(1. / 276_480., t5_10 / 1.35168e+06),
            );
            u -= t6_6.mul_add(
                1. / 322_560.,
                t6_8.mul_add(1. / 1.65888e+06, t6_10 / 8.11008e+06),
            );

            v -= t7_8.mul_add(1. / 1.16122e+07, t7_10 / 5.67706e+07);
            u += t8_8.mul_add(1. / 9.28973e+07, t8_10 / 4.54164e+08);

            v += t9_10 / 4.08748e+09;
            u -= t10_10 / 4.08748e+10;

            let th = th4.mul_add(s, th3).mul_add(s, th2).mul_add(s, th1) * s;
            let cth = th.cos();
            let sth = th.sin();

            x += cth * u - sth * v;
            y += cth * v + sth * u;
            s += ds;
        }

        (x * ds, y * ds)
    }

    #[must_use]
    pub fn bend(&self) -> f64 {
        let [k0, k1, k2, k3] = self.0;
        k0.abs() + (k1 / 2.).abs() + (k2 / 8.).abs() + (k3 / 48.).abs()
    }

    #[must_use]
    pub fn theta(&self, s0: f64, s1: f64) -> f64 {
        let [k0, k1, k2, k3] = self.0;
        let s = s0.midpoint(s1);

        (k3 / 24.)
            .mul_add(s, k2 / 6.)
            .mul_add(s, k1 / 2.)
            .mul_add(s, k0)
            * s
    }

    #[must_use]
    pub fn divide(&self, s0: f64, s1: f64) -> Self {
        let [k0, k1, k2, k3] = self.0;
        let s = s0.midpoint(s1);
        let t = s1 - s0;

        Self([
            t * (k3 / 6.).mul_add(s, k2 / 2.).mul_add(s, k1).mul_add(s, k0),
            t * t * (k3 / 2.).mul_add(s, k2).mul_add(s, k1),
            t * t * t * k3.mul_add(s, k2),
            t * t * t * t * k3,
        ])
    }
}
