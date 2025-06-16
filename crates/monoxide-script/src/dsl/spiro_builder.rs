use std::sync::Arc;

use monoxide_curves::{
    point::Point2D,
    stroke::{Tangent, TangentOverride},
};
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use super::IntoOutline;
use crate::ast::OutlineExpr;

#[derive(Debug, Clone)]
pub struct SpiroBuilder {
    points: Vec<SpiroCp>,
    tangent_override: TangentOverride,
    is_closed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpiroInst {
    pt: SpiroCp,
    opts: SpiroInstOpts,
}

impl From<SpiroCp> for SpiroInst {
    fn from(pt: SpiroCp) -> Self {
        Self {
            pt,
            opts: SpiroInstOpts::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SpiroInstOpts {
    heading: Option<Point2D>,
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::Corner`].
///
/// # Examples
/// ```
/// # use monoxide_script::corner;
/// let p = corner!(3., 4.);
/// # _ = corner!(3., 4.,);
/// let pair = (3., 4.);
/// let q = corner!(pair);
/// # _ = corner!(pair,);
/// assert_eq!(p, q);
/// ```
#[macro_export]
macro_rules! corner {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::Corner.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::G4`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! g4 {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::G4.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::G2`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! g2 {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::G2.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::Left`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! flat {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::Left.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::Right`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! curl {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::Right.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::Anchor`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! anchor {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::Anchor.at(p)), $($body)+)
    };
}

/// Convenience macro to create a [`SpiroInst`] of type [`SpiroCpTy::Handle`].
///
/// See [`corner`] for usage examples of this macro.
#[macro_export]
macro_rules! handle {
    ($($body:tt)+) => {
        $crate::ctrl_pt!(|p| $crate::dsl::SpiroInst::from(::monoxide_spiro::SpiroCpTy::Handle.at(p)), $($body)+)
    };
}

impl SpiroInst {
    pub fn heading(mut self, pt: impl Into<Point2D>) -> Self {
        self.opts.heading = Some(pt.into());
        self
    }
}

impl SpiroBuilder {
    pub fn open() -> Self {
        Self::new(false)
    }

    pub fn closed() -> Self {
        Self::new(true)
    }

    fn new(is_closed: bool) -> Self {
        Self {
            is_closed,
            points: vec![],
            tangent_override: TangentOverride::default(),
        }
    }

    pub fn inst(mut self, inst: SpiroInst) -> Self {
        self.points.push(inst.pt);
        if let Some(tan) = inst.opts.heading {
            let tan = Some(tan.normalize());
            self.tangent_override
                .insert(self.points.len() - 1, Tangent { in_: tan, out: tan });
        }
        self
    }

    pub fn insts(mut self, insts: impl IntoIterator<Item = SpiroInst>) -> Self {
        for inst in insts {
            self = self.inst(inst);
        }
        self
    }

    pub fn build(mut self) -> OutlineExpr {
        if !self.is_closed && !self.points.is_empty() {
            self.points.last_mut().unwrap().ty = SpiroCpTy::EndOpen;
            self.points.first_mut().unwrap().ty = SpiroCpTy::Open;
        }
        OutlineExpr::Spiro(self.points, self.tangent_override)
    }
}

impl IntoOutline for SpiroBuilder {
    fn into_outline(self) -> Arc<OutlineExpr> {
        Arc::new(self.build())
    }
}
