pub mod bez;
mod error;
pub mod spiro;

use crate::{bez::Ctx, spiro::Segs};
pub use crate::{
    error::{Error, Result},
    spiro::{Cp, CpTy},
};

pub type Point = (f64, f64);

/// Renders a spiro curve to a bezier context.
///
/// # Errors
///
/// Returns [`Error::NotEnoughCps`] if `cps` has fewer than 2 elements.
pub fn bezier<C: Ctx>(
    cps: impl IntoIterator<Item = Cp>,
    ctx: &mut C,
    is_closed: bool,
    delta: impl Into<Option<f64>>,
) -> Result<()> {
    let delta = delta.into().unwrap_or(1.);
    let mut cps: Vec<_> = cps.into_iter().collect();

    ctx.start();
    let segs = Segs::from_cps(&mut cps, is_closed)?;
    segs.render_bez(ctx, segs.0.len(), delta);
    ctx.end();

    Ok(())
}
