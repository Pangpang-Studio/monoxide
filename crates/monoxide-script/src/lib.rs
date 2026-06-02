pub mod ast;
pub mod dsl;
pub mod eval;
pub mod prelude;
pub mod trace;

/// Settings required for font evaluation.
///
/// This trait defines the essential parameters needed to populate TrueType
/// font metrics metadata. Note that other params like font name are defined in
/// [`eval::AuxiliarySettings`].
pub trait EvalSettings: std::fmt::Debug + Send + Sync {
    /// The full width of a half-width monospace character.
    fn mono_width(&self) -> f64;

    /// The cap height (height of capital letters).
    fn cap_height(&self) -> f64;

    /// The descender (usually a negative value).
    fn descender(&self) -> f64;

    /// The x-height (height of lowercase letters like 'x').
    fn x_height(&self) -> f64;
}
