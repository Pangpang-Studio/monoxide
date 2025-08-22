pub mod ast;
pub mod dsl;
pub mod eval;
pub mod trace;

#[doc(hidden)]
#[macro_export]
macro_rules! __let_settings_single {
    ($meth:ident, $target:expr) => {
        let $meth = $target.$meth();
    };
    ($meth:ident, $var:ident, $target:expr) => {
        let $var = $target.$meth();
    };
}

/// Convenience macro to extract original or derived values from
/// a settings struct.
///
/// # Examples
///
/// ```no_run
/// # use monoxide_script::let_settings;
/// # struct Foo;
/// # impl Foo {
/// #   fn mid(&self){}
/// #   fn mih(&self){}
/// #   fn ovs(&self){}
/// # }
/// # let s: Foo = todo!();
/// let_settings! { { mid, mih, ovs: o } = s; }
/// ```
///
/// ... will expand to:
///
/// ```no_run
/// # struct Foo;
/// # impl Foo {
/// #   fn mid(&self){}
/// #   fn mih(&self){}
/// #   fn ovs(&self){}
/// # }
/// # let s: Foo = todo!();
/// let mid = s.mid();
/// let mih = s.mih();
/// let o = s.ovs();
/// ```
#[macro_export]
macro_rules! let_settings {
    (
        { $( $meth:ident $( : $var:ident )? ),+ $(,)? } = $target:expr ;
    ) => {
        $(
            $crate::__let_settings_single!($meth $(, $var)? , $target);
        )+
    };
}

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
