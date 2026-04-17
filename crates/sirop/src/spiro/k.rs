use crate::Point;

#[derive(Default, Debug, Clone, Copy)]
pub struct K(pub [f64; 4]);

impl K {
    /// Integrate polynomial spiral curve from -0.5 to 0.5.
    #[allow(clippy::many_single_char_names)]
    #[must_use]
    pub fn integrate_spiro(&self) -> Point {
        let [k0, k1, k2, k3] = self.0;
        let iterations = 4;

        let th1 = k0;
        let th2 = 0.5 * k1;
        let th3 = (1. / 6.) * k2;
        let th4 = (1. / 24.) * k3;
        let ds = 1. / f64::from(iterations);
        let ds2 = ds * ds;
        let ds3 = ds2 * ds;
        let k0 = k0 * ds;
        let k1 = k1 * ds;
        let k2 = k2 * ds;
        let k3 = k3 * ds;

        let mut s = ds.mul_add(0.5, -0.5);

        let mut x = 0.;
        let mut y = 0.;
        for _ in 0..iterations {
            let km0 = (1. / 6. * k3)
                .mul_add(s, 0.5 * k2)
                .mul_add(s, k1)
                .mul_add(s, k0);
            let km1 = (0.5 * k3).mul_add(s, k2).mul_add(s, k1) * ds;
            let km2 = k3.mul_add(s, k2) * ds2;
            let km3 = k3 * ds3;

            // Order 12 implementation
            let t1_1 = km0;
            let t1_2 = 0.5 * km1;
            let t1_3 = 1. / 6. * km2;
            let t1_4 = 1. / 24. * km3;
            let t2_2 = t1_1 * t1_1;
            let t2_3 = 2. * (t1_1 * t1_2);
            let t2_4 = 2_f64.mul_add(t1_1 * t1_3, t1_2 * t1_2);
            let t2_5 = 2. * (t1_1 * t1_4 + t1_2 * t1_3);
            let t2_6 = 2_f64.mul_add(t1_2 * t1_4, t1_3 * t1_3);
            let t2_7 = 2. * (t1_3 * t1_4);
            let t2_8 = t1_4 * t1_4;
            let t3_4 = t2_2 * t1_2 + t2_3 * t1_1;
            let t3_6 = t2_2 * t1_4 + t2_3 * t1_3 + t2_4 * t1_2 + t2_5 * t1_1;
            let t3_8 = t2_4 * t1_4 + t2_5 * t1_3 + t2_6 * t1_2 + t2_7 * t1_1;
            let t3_10 = t2_6 * t1_4 + t2_7 * t1_3 + t2_8 * t1_2;
            let t4_4 = t2_2 * t2_2;
            let t4_5 = 2. * (t2_2 * t2_3);
            let t4_6 = 2_f64.mul_add(t2_2 * t2_4, t2_3 * t2_3);
            let t4_7 = 2. * (t2_2 * t2_5 + t2_3 * t2_4);
            let t4_8 = 2_f64.mul_add(t2_2 * t2_6 + t2_3 * t2_5, t2_4 * t2_4);
            let t4_9 = 2. * (t2_2 * t2_7 + t2_3 * t2_6 + t2_4 * t2_5);
            let t4_10 = 2_f64.mul_add(t2_2 * t2_8 + t2_3 * t2_7 + t2_4 * t2_6, t2_5 * t2_5);
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

            v += (1_f64 / 12.).mul_add(t1_2, 1. / 80. * t1_4);
            u -= (1_f64 / 24.).mul_add(
                t2_2,
                (1_f64 / 160.).mul_add(t2_4, (1_f64 / 896.).mul_add(t2_6, 1_f64 / 4608. * t2_8)),
            );

            v -= (1_f64 / 480.).mul_add(
                t3_4,
                (1_f64 / 2688.).mul_add(t3_6, (1_f64 / 13824.).mul_add(t3_8, 1. / 67584. * t3_10)),
            );
            u += (1_f64 / 1920.).mul_add(
                t4_4,
                (1_f64 / 10752.)
                    .mul_add(t4_6, (1_f64 / 55296.).mul_add(t4_8, 1. / 270_336. * t4_10)),
            );

            v += (1_f64 / 53760.).mul_add(
                t5_6,
                (1_f64 / 276_480.).mul_add(t5_8, 1. / 1.35168e+06 * t5_10),
            );
            u -= (1_f64 / 322_560.).mul_add(
                t6_6,
                (1_f64 / 1.65888e+06).mul_add(t6_8, 1. / 8.11008e+06 * t6_10),
            );

            v -= (1_f64 / 1.16122e+07).mul_add(t7_8, 1. / 5.67706e+07 * t7_10);
            u += (1_f64 / 9.28973e+07).mul_add(t8_8, 1. / 4.54164e+08 * t8_10);

            v += 1. / 4.08748e+09 * t9_10;
            u -= 1. / 4.08748e+10 * t10_10;

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
        k0.abs() + (0.5 * k1).abs() + (0.125 * k2).abs() + (1. / 48. * k3).abs()
    }

    #[must_use]
    pub fn theta(&self, s0: f64, s1: f64) -> f64 {
        let [k0, k1, k2, k3] = self.0;
        let s = s0.midpoint(s1);

        (1. / 24. * k3)
            .mul_add(s, 1. / 6. * k2)
            .mul_add(s, 1. / 2. * k1)
            .mul_add(s, k0)
            * s
    }

    #[must_use]
    pub fn divide(&self, s0: f64, s1: f64) -> Self {
        let [k0, k1, k2, k3] = self.0;
        let s = s0.midpoint(s1);
        let t = s1 - s0;

        Self([
            t * (1. / 6. * k3)
                .mul_add(s, 1. / 2. * k2)
                .mul_add(s, k1)
                .mul_add(s, k0),
            t * t * (1. / 2. * k3).mul_add(s, k2).mul_add(s, k1),
            t * t * t * k3.mul_add(s, k2),
            t * t * t * t * k3,
        ])
    }
}
