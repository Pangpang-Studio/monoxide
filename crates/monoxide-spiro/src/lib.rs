mod bez;

use std::mem;

use spiro_sys::{
    SPIRO_ANCHOR, SPIRO_CORNER, SPIRO_END_OPEN_CONTOUR, SPIRO_G2, SPIRO_G4, SPIRO_HANDLE,
    SPIRO_LEFT, SPIRO_OPEN_CONTOUR, SPIRO_RIGHT, spiro_cp,
};

pub use self::bez::BezCtx;

/// A Spiro control point.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct SpiroCp {
    pub x: f64,
    pub y: f64,
    pub ty: SpiroCpTy,
}

impl SpiroCp {
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _HAS_SAME_LAYOUT_AS_SPIRO_SYS_CP: () = {
        ["Size of SpiroCp"][mem::size_of::<spiro_cp>() - mem::size_of::<SpiroCp>()];
        ["Alignment of SpiroCp"][mem::align_of::<spiro_cp>() - mem::align_of::<SpiroCp>()];
        ["Offset of field: SpiroCp::x"][mem::offset_of!(spiro_cp, x) - mem::offset_of!(SpiroCp, x)];
        ["Offset of field: SpiroCp::y"][mem::offset_of!(spiro_cp, y) - mem::offset_of!(SpiroCp, y)];
        ["Offset of field: SpiroCp::ty"]
            [mem::offset_of!(spiro_cp, ty) - mem::offset_of!(SpiroCp, ty)];
    };
}

impl From<spiro_cp> for SpiroCp {
    fn from(cp: spiro_cp) -> Self {
        // SAFETY: This is safe because the two types have the same layout,
        // as guaranteed by the static assertions above.
        unsafe { mem::transmute(cp) }
    }
}

impl From<SpiroCp> for spiro_cp {
    fn from(cp: SpiroCp) -> Self {
        // SAFETY: This is safe because the two types have the same layout,
        // as guaranteed by the static assertions above.
        unsafe { mem::transmute(cp) }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SpiroCpTy {
    #[default]
    Corner = SPIRO_CORNER,
    G4 = SPIRO_G4,
    G2 = SPIRO_G2,
    /// Also known as "flat".
    Left = SPIRO_LEFT,
    /// Also known as "curl".
    Right = SPIRO_RIGHT,
    Anchor = SPIRO_ANCHOR,
    Handle = SPIRO_HANDLE,
    Open = SPIRO_OPEN_CONTOUR,
    EndOpen = SPIRO_END_OPEN_CONTOUR,
}
