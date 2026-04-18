mod bez;

pub use sirop::Error;

pub use self::bez::BezCtx;

/// A Spiro control point.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SpiroCp {
    pub x: f64,
    pub y: f64,
    pub ty: SpiroCpTy,
}

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

impl From<SpiroCpTy> for sirop::CpTy {
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
