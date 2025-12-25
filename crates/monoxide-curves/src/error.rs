use std::borrow::Cow;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("spiro curve could not be converted to cubic bezier curves")]
    SpiroInconvertible,

    #[error("spiro curve is not a single piece")]
    SpiroBroken,

    #[error("internal error: {0}")]
    Internal(Cow<'static, str>),
}

impl Error {
    pub fn internal<S: Into<Cow<'static, str>>>(msg: S) -> Self {
        Error::Internal(msg.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
