//! Operations and types related to Bezier curves.
//!
//! This crate is a thin wrapper around [`kurbo`]'s curve and geometry types.
//! It re-exports [`kurbo::Point`] as [`point::Point2D`] and layers a small
//! amount of domain-specific functionality on top of `kurbo`, most notably
//! stroking a curve with per-point width/alignment control (see [`stroke`]).
pub mod convert;
pub mod cube;
pub mod debug;
pub mod error;
pub mod point;
pub mod quad;
pub mod spiro;
pub mod stroke;
pub mod xform;
pub use cube::{CubicBezier, CubicSegment};
pub use quad::{QuadBezier, QuadSegment};

/// Represents a spiro curve.
pub type SpiroCurve = spiro::SpiroCurve;
