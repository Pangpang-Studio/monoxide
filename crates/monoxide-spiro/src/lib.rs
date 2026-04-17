mod bez;

#[cfg(feature = "backend-sirop")]
use sirop::CpTy;
#[cfg(feature = "backend-spiro-sys")]
use spiro_sys::{
    SPIRO_CORNER, SPIRO_END_OPEN_CONTOUR, SPIRO_G2, SPIRO_G4, SPIRO_LEFT, SPIRO_OPEN_CONTOUR,
    SPIRO_RIGHT,
};

pub use self::bez::BezCtx;

/// A Spiro control point.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SpiroCp {
    pub x: f64,
    pub y: f64,
    pub ty: SpiroCpTy,
}

#[cfg(feature = "backend-spiro-sys")]
impl From<spiro_sys::spiro_cp> for SpiroCp {
    fn from(cp: spiro_sys::spiro_cp) -> Self {
        Self {
            x: cp.x,
            y: cp.y,
            ty: SpiroCpTy::from_sys(cp.ty as _),
        }
    }
}

#[cfg(feature = "backend-spiro-sys")]
impl From<SpiroCp> for spiro_sys::spiro_cp {
    fn from(cp: SpiroCp) -> Self {
        Self {
            x: cp.x,
            y: cp.y,
            ty: cp.ty.to_sys() as _,
        }
    }
}

#[cfg(feature = "backend-sirop")]
impl From<SpiroCp> for sirop::Cp {
    fn from(cp: SpiroCp) -> Self {
        Self {
            pt: (cp.x, cp.y),
            ty: cp.ty.into(),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SpiroCpTy {
    #[default]
    Corner,
    G4,
    G2,
    /// Also known as "flat".
    Left,
    /// Also known as "curl".
    Right,
    Open,
    EndOpen,
}

#[cfg(feature = "backend-spiro-sys")]
impl SpiroCpTy {
    const fn from_sys(val: u8) -> Self {
        match val {
            SPIRO_CORNER => Self::Corner,
            SPIRO_G4 => Self::G4,
            SPIRO_G2 => Self::G2,
            SPIRO_LEFT => Self::Left,
            SPIRO_RIGHT => Self::Right,
            SPIRO_OPEN_CONTOUR => Self::Open,
            SPIRO_END_OPEN_CONTOUR => Self::EndOpen,
            _ => unreachable!(),
        }
    }

    const fn to_sys(self) -> u8 {
        match self {
            Self::Corner => SPIRO_CORNER,
            Self::G4 => SPIRO_G4,
            Self::G2 => SPIRO_G2,
            Self::Left => SPIRO_LEFT,
            Self::Right => SPIRO_RIGHT,
            Self::Open => SPIRO_OPEN_CONTOUR,
            Self::EndOpen => SPIRO_END_OPEN_CONTOUR,
        }
    }
}

#[cfg(feature = "backend-sirop")]
impl From<SpiroCpTy> for CpTy {
    fn from(val: SpiroCpTy) -> Self {
        match val {
            SpiroCpTy::Corner => Self::Corner,
            SpiroCpTy::G4 => Self::G4,
            SpiroCpTy::G2 => Self::G2,
            SpiroCpTy::Left => Self::Flat,
            SpiroCpTy::Right => Self::Curl,
            SpiroCpTy::Open => Self::Open,
            SpiroCpTy::EndOpen => Self::EndOpen,
        }
    }
}

impl SpiroCpTy {
    pub fn at(self, pt: impl Into<(f64, f64)>) -> SpiroCp {
        let (x, y) = pt.into();
        SpiroCp { x, y, ty: self }
    }
}
