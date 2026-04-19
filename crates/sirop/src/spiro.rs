mod arc;
mod k;

pub use self::arc::Arc;
use self::k::K;
use crate::{Error, Point, Result, bez::Ctx};

/// A spiro control point.
#[derive(Default, Debug, Clone, Copy)]
pub struct Cp {
    pub pt: Point,
    pub ty: CpTy,
}

impl Cp {
    const fn x(&self) -> f64 {
        self.pt.0
    }

    const fn y(&self) -> f64 {
        self.pt.1
    }
}

/// The type of a [`Cp`].
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum CpTy {
    #[default]
    Corner = b'v',
    G4 = b'o',
    G2 = b'c',
    /// Also known as "left".
    Flat = b'[',
    /// Also known as "right".
    Curl = b']',
    Open = b'{',
    EndOpen = b'}',
}

impl CpTy {
    const fn jinc(self, other: Self) -> usize {
        #[allow(clippy::enum_glob_use)]
        use CpTy::*;

        match (self, other) {
            (G4 | Curl, _) | (_, G4 | Flat) => 4,
            (G2, G2) => 2,
            (G2, EndOpen | Corner | Curl) | (Open | Corner | Flat, G2) => 1,
            _ => 0,
        }
    }
}

impl TryFrom<u8> for CpTy {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'v' => Ok(Self::Corner),
            b'o' => Ok(Self::G4),
            b'c' => Ok(Self::G2),
            b'[' => Ok(Self::Flat),
            b']' => Ok(Self::Curl),
            b'{' => Ok(Self::Open),
            b'}' => Ok(Self::EndOpen),
            i => Err(i),
        }
    }
}

/// A spiro [`Seg`]ment between two control points.
#[derive(Default, Debug, Clone, Copy)]
pub struct Seg {
    cp: Cp,

    bend_th: f64,
    ks: K,
    ch: f64,
    th: f64,
}

impl Seg {
    const fn pt(&self) -> Point {
        self.cp.pt
    }

    const fn x(&self) -> f64 {
        self.cp.x()
    }

    const fn y(&self) -> f64 {
        self.cp.y()
    }

    const fn ty(&self) -> CpTy {
        self.cp.ty
    }

    fn ends_and_pderivs(&self, jinc: usize, ends: &mut Ends, derivs: &mut Derivs) {
        let recip_delta = 2e6_f64;
        let delta = recip_delta.recip();
        let mut try_ends = Ends::default();
        self.ks.ends(self.ch, ends);

        #[allow(clippy::needless_range_loop)]
        for i in 0..jinc {
            let mut try_ks = self.ks;
            try_ks.0[i] += delta;
            try_ks.ends(self.ch, &mut try_ends);
            for k in 0..2 {
                for j in 0..4 {
                    derivs[j][k][i] = recip_delta * (try_ends[k][j] - ends[k][j]);
                }
            }
        }
    }
}

/// An ordered collection of spiro [`Seg`]ments.
#[derive(Debug)]
pub struct Segs(pub Vec<Seg>);

impl Segs {
    fn new(src: &[Cp]) -> Self {
        let n = src.len();
        let is_open = src[0].ty == CpTy::Open;
        let n_seg = n - usize::from(is_open);

        let mut r = vec![Seg::default(); n_seg + 1];
        for (cp, s) in src[..n_seg].iter().zip(r.iter_mut()) {
            s.cp = *cp;
        }
        r[n_seg].cp = src[n_seg % n];

        for i in 0..n_seg {
            let dx = r[i + 1].x() - r[i].x();
            let dy = r[i + 1].y() - r[i].y();
            r[i].ch = dx.hypot(dy);
            r[i].th = dy.atan2(dx);
        }

        let mut i_last = n_seg - 1;
        for i in 0..n_seg {
            r[i].bend_th = match r[i].ty() {
                CpTy::Open | CpTy::EndOpen | CpTy::Corner => 0.,
                _ => mod_tau(r[i].th - r[i_last].th),
            };
            i_last = i;
        }

        Self(r)
    }

    fn count(&self, n_seg: usize) -> usize {
        self.0[..=n_seg]
            .array_windows()
            .map(|[w0, w1]| w0.ty().jinc(w1.ty()))
            .sum()
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::too_many_lines,
        clippy::similar_names
    )]
    fn spiro(&mut self, n: usize, m: &mut [BandMat], perm: &mut [usize], v: &mut [f64]) -> f64 {
        let nmat = self.count(n);
        let s = &mut self.0;
        let is_cyclic = !matches!(s[0].ty(), CpTy::Open | CpTy::Corner);

        m.fill(BandMat::default());
        v.fill(0.);

        let mut i = 0;
        let mut j = 0;
        let mut jj = match s[0].ty() {
            CpTy::G4 => nmat - 2,
            CpTy::G2 => nmat - 1,
            _ => 0,
        };
        if is_cyclic && s[0].ty() == CpTy::Flat {
            let mut scanned_i = 0;
            let mut scanned_j = 0;
            let mut scanned_jj = 0;
            let mut found = false;
            while scanned_i < n {
                let ty0 = s[scanned_i].ty();
                let ty1 = s[scanned_i + 1].ty();
                match ty0 {
                    CpTy::G4 => {
                        scanned_jj -= 2;
                        found = true;
                    }
                    CpTy::G2 => {
                        scanned_jj -= 1;
                        found = true;
                    }
                    CpTy::Corner => {
                        found = true;
                    }
                    _ => {
                        let inc = ty0.jinc(ty1);
                        scanned_j += inc;
                        scanned_jj += inc.cast_signed();
                        scanned_i += 1;
                        continue;
                    }
                }
                scanned_j %= nmat;
                scanned_jj = scanned_jj.rem_euclid(nmat.cast_signed());
                i = scanned_i;
                j = scanned_j;
                jj = scanned_jj.cast_unsigned();
                break;
            }
            if !found {
                i = 0;
                j = 0;
                jj = 0;
            }
        }

        for _ in 0..n {
            i %= n;
            let ty0 = s[i].ty();
            let ty1 = s[i + 1].ty();
            let jinc = ty0.jinc(ty1);
            let th = s[i].bend_th;
            let mut ends = Ends::default();
            let mut derivs = Derivs::default();

            let mut jthl = usize::MAX;
            let mut jk0l = usize::MAX;
            let mut jk1l = usize::MAX;
            let mut jk2l = usize::MAX;
            let mut jthr = usize::MAX;
            let mut jk0r = usize::MAX;
            let mut jk1r = usize::MAX;
            let mut jk2r = usize::MAX;

            s[i].ends_and_pderivs(jinc, &mut ends, &mut derivs);

            /* constraints crossing LEFT */
            if matches!(ty0, CpTy::G4 | CpTy::G2 | CpTy::Flat | CpTy::Curl) {
                jthl = jj % nmat;
                jj += 1;
                jj %= nmat;
                jk0l = jj;
                jj += 1;
            }
            if ty0 == CpTy::G4 {
                jj %= nmat;
                jk1l = jj;
                jj += 1;
                jk2l = jj;
                jj += 1;
            }

            /* constraints on LEFT */
            if matches!(ty0, CpTy::Flat | CpTy::Corner | CpTy::Open | CpTy::G2) && jinc == 4 {
                if ty0 != CpTy::G2 {
                    jk1l = jj;
                    jj += 1;
                }
                jk2l = jj;
                jj += 1;
            }

            /* constraints on RIGHT */
            if matches!(ty1, CpTy::Curl | CpTy::Corner | CpTy::EndOpen | CpTy::G2) && jinc == 4 {
                if ty1 != CpTy::G2 {
                    jk1r = jj;
                    jj += 1;
                }
                jk2r = jj;
                jj += 1;
            }

            /* constraints crossing RIGHT */
            if matches!(ty1, CpTy::G4 | CpTy::G2 | CpTy::Flat | CpTy::Curl) {
                jj %= nmat;
                jthr = jj;
                jk0r = (jj + 1) % nmat;
            }
            if ty1 == CpTy::G4 {
                jk1r = (jj + 2) % nmat;
                jk2r = (jj + 3) % nmat;
            }

            let mut add_mat_line = |derivs: &[f64], x: f64, y: f64, jj: usize| {
                if jj == usize::MAX {
                    return;
                }
                let jj = jj % nmat;
                let joff = match nmat {
                    ..6 => j + 5 - jj,
                    6 => 2 + ((j + 3 + nmat - jj) % nmat),
                    7.. => (j + 5 + nmat - jj) % nmat,
                };
                v[jj] += x;
                for (k, &deriv) in derivs[..jinc].iter().enumerate() {
                    m[jj].a[joff + k] += y * deriv;
                }
            };

            add_mat_line(&derivs[0][0], th - ends[0][0], 1., jthl);
            add_mat_line(&derivs[1][0], ends[0][1], -1., jk0l);
            add_mat_line(&derivs[2][0], ends[0][2], -1., jk1l);
            add_mat_line(&derivs[3][0], ends[0][3], -1., jk2l);
            add_mat_line(&derivs[0][1], -ends[1][0], 1., jthr);
            add_mat_line(&derivs[1][1], -ends[1][1], 1., jk0r);
            add_mat_line(&derivs[2][1], -ends[1][2], 1., jk1r);
            add_mat_line(&derivs[3][1], -ends[1][3], 1., jk2r);

            if jthl != usize::MAX {
                v[jthl] = mod_tau(v[jthl]);
            }
            if jthr != usize::MAX {
                v[jthr] = mod_tau(v[jthr]);
            }

            j = (j + jinc) % nmat;
            i += 1;
        }

        let n_invert;
        (n_invert, j) = if is_cyclic {
            m.copy_within(0..nmat, nmat);
            m.copy_within(0..nmat, 2 * nmat);
            v.copy_within(0..nmat, nmat);
            v.copy_within(0..nmat, 2 * nmat);
            (3 * nmat, nmat)
        } else {
            (nmat, 0)
        };

        BandMat::bandec11(m, n_invert, perm);
        BandMat::banbks11(m, n_invert, perm, v);

        let mut norm = 0.;
        for i in 0..n {
            let ty0 = s[i].ty();
            let ty1 = s[i + 1].ty();
            for k in 0..ty0.jinc(ty1) {
                let dk = v[j];
                j += 1;
                s[i].ks.0[k] += dk;
                norm += dk * dk;
            }
        }
        norm
    }

    fn solve(&mut self, n_seg: usize) {
        let nmat = self.count(n_seg);
        if nmat == 0 {
            return;
        }

        let s = &mut self.0;
        let mut n_alloc = nmat;
        if !matches!(s[0].ty(), CpTy::Open | CpTy::Corner) {
            n_alloc *= 3;
        }
        n_alloc = n_alloc.max(5);

        let mut m = vec![BandMat::default(); n_alloc];
        let mut v = vec![0.; n_alloc];
        let mut perm = vec![0; n_alloc];

        for _ in 0..10 {
            let norm = self.spiro(n_seg, &mut m, &mut perm, &mut v);
            if norm < 1e-12 {
                break;
            }
        }
    }

    pub(crate) fn render_bez<C: Ctx>(&self, ctx: &mut C, n_seg: usize, delta: f64) {
        let s = &self.0;
        let n_seg = n_seg - usize::from(s[0].ty() == CpTy::Open);

        for (i, [p0, p1]) in s.array_windows().enumerate().take(n_seg) {
            let (p0, p1) = (p0.pt(), p1.pt());

            if i == 0 {
                ctx.move_to(p0);
            }

            ctx.mark_knot(i);
            Arc::new(s[i].ks, p0, p1).render_bez(ctx, delta);
        }
    }

    /// Creates a [`Segs`] from a list of control points.
    ///
    /// If `is_closed` is `false`, the [`CpTy`]s of the first and the last
    /// control points will be replaced with [`CpTy::Open`] and
    /// [`CpTy::EndOpen`] respectively before calculating the segments.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotEnoughCps`] if `cps` has fewer than 2 elements.
    pub fn from_cps(cps: &mut [Cp], is_closed: bool) -> Result<Self> {
        let n = cps.len();
        let [head, .., last] = cps else {
            return Err(Error::NotEnoughCps);
        };
        if !is_closed {
            head.ty = CpTy::Open;
            last.ty = CpTy::EndOpen;
        }

        let n_seg = n - usize::from(head.ty == CpTy::Open);
        let mut segs = Self::new(cps);
        segs.solve(n_seg);
        Ok(segs)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct BandMat {
    /// The band-diagonal matrix.
    a: [f64; 11],
    /// The lower part of band-diagonal decomposition.
    al: [f64; 5],
}

impl BandMat {
    fn bandec11(m: &mut [Self], n: usize, perm: &mut [usize]) {
        // Pack top triangle to the LEFT.
        for (i, mi) in m[..5].iter_mut().enumerate() {
            for j in 0..(i + 6) {
                mi.a[j] = mi.a[j + 5 - i];
            }
            for it in &mut mi.a[i + 6..] {
                *it = 0.;
            }
        }

        let mut l = 5;
        for k in 0..n {
            let mut pivot = k;
            let mut pivot_val = m[k].a[0];

            l = if l < n { 1 + l } else { n };

            for (j, mj) in m[..l].iter().enumerate().skip(k + 1) {
                if mj.a[0].abs() > pivot_val.abs() {
                    pivot_val = mj.a[0];
                    pivot = j;
                }
            }

            perm[k] = pivot;
            m.swap(k, pivot);

            if pivot_val.abs() < 1e-12 {
                pivot_val = 1e-12;
            }
            let pivot_scale = pivot_val.recip();
            for i in (k + 1)..l {
                let x = m[i].a[0] * pivot_scale;
                m[k].al[i - k - 1] = x;
                for j in 1..11 {
                    m[i].a[j - 1] = x.mul_add(-m[k].a[j], m[i].a[j]);
                }
                m[i].a[10] = 0.;
            }
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn banbks11(m: &[Self], n: usize, perm: &[usize], v: &mut [f64]) {
        /* forward substitution */
        let mut l = 5;
        for k in 0..n {
            let i = perm[k];
            v.swap(i, k);
            l += usize::from(l < n);
            for i in (k + 1)..l {
                v[i] -= m[k].al[i - k - 1] * v[k];
            }
        }

        /* back substitution */
        l = 1;
        for i in (0..n).rev() {
            let mut x = v[i];
            for k in 1..l {
                x -= m[i].a[k] * v[k + i];
            }
            v[i] = x / m[i].a[0];
            l += usize::from(l < 11);
        }
    }
}

type Ends = [[f64; 4]; 2];
type Derivs = [[[f64; 4]; 2]; 4];

impl K {
    fn ends(&self, seg_ch: f64, ends: &mut Ends) -> f64 {
        let [k0, k1, k2, k3] = self.0;

        let (sx, sy) = self.integrate_spiro();
        let ch = sx.hypot(sy);
        let th = sy.atan2(sx);
        let l = ch / seg_ch;

        let th_even = k0 / 2. + k2 / 48.;
        let th_odd = k1 / 8. + k3 / 384. - th;
        ends[0][0] = th_even - th_odd;
        ends[1][0] = th_even + th_odd;

        let k0_even = l * (k0 + k2 / 8.);
        let k0_odd = l * (k1 / 2. + k3 / 48.);
        ends[0][1] = k0_even - k0_odd;
        ends[1][1] = k0_even + k0_odd;

        let l2 = l * l;
        let k1_even = l2 * (k1 + k3 / 8.);
        let k1_odd = l2 * k2 / 2.;
        ends[0][2] = k1_even - k1_odd;
        ends[1][2] = k1_even + k1_odd;

        let l3 = l2 * l;
        let k2_even = l3 * k2;
        let k2_odd = l3 * k3 / 2.;
        ends[0][3] = k2_even - k2_odd;
        ends[1][3] = k2_even + k2_odd;

        l
    }
}

fn mod_tau(th: f64) -> f64 {
    use std::f64::consts::TAU;

    let u = th / TAU;
    TAU * (u - (u + 0.5).floor())
}
