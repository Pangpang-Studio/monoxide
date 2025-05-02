use std::ffi::c_int;

use spiro_sys::{SpiroCPsToBezier2, bezctx, spiro_cp};

use crate::{SpiroCp, SpiroCpTy};

pub trait BezCtx {
    fn move_to(&mut self, x: f64, y: f64, is_open: bool);
    fn line_to(&mut self, x: f64, y: f64);
    fn quad_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64);
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

        self.start();
        let mut ctx = BezCtxAdapter::new(self);
        let mut ps = ps.iter().cloned().map(spiro_cp::from).collect::<Vec<_>>();
        let ncq = 0;
        let ok = (unsafe {
            SpiroCPsToBezier2(
                ps.as_mut_ptr(),
                ps.len() as _,
                ncq,
                is_closed as _,
                (&raw mut ctx).cast(),
            )
        }) == 1;
        self.end();

        ok
    }
}

#[repr(C)]
struct BezCtxAdapter<'a, T> {
    raw: bezctx,
    data: &'a mut T,
}

impl<T: BezCtx> BezCtxAdapter<'_, T> {
    const RAW: bezctx = {
        bezctx {
            moveto: Some(Self::_move_to),
            lineto: Some(Self::_line_to),
            quadto: Some(Self::_quad_to),
            curveto: Some(Self::_curve_to),
            mark_knot: Some(Self::_mark_knot),
        }
    };

    fn new(data: &mut T) -> BezCtxAdapter<'_, T> {
        BezCtxAdapter {
            raw: Self::RAW,
            data,
        }
    }

    unsafe extern "C" fn _move_to(bc: *mut bezctx, x: f64, y: f64, is_open: c_int) {
        let this = unsafe { &mut *(bc as *mut Self) };
        this.data.move_to(x, y, is_open != 0);
    }

    unsafe extern "C" fn _line_to(bc: *mut bezctx, x: f64, y: f64) {
        let this = unsafe { &mut *(bc as *mut Self) };
        this.data.line_to(x, y);
    }

    unsafe extern "C" fn _quad_to(bc: *mut bezctx, x1: f64, y1: f64, x2: f64, y2: f64) {
        let this = unsafe { &mut *(bc as *mut Self) };
        this.data.quad_to(x1, y1, x2, y2);
    }

    unsafe extern "C" fn _curve_to(
        bc: *mut bezctx,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    ) {
        let this = unsafe { &mut *(bc as *mut Self) };
        this.data.curve_to(x1, y1, x2, y2, x3, y3);
    }

    unsafe extern "C" fn _mark_knot(bc: *mut bezctx, knot_idx: c_int) {
        let this = unsafe { &mut *(bc as *mut Self) };
        this.data.mark_knot(knot_idx as usize);
    }
}
