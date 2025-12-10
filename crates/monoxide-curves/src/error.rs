#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("spiro curve could not be converted to cubic bezier curves")]
    SpiroInconvertible,

    #[error("spiro curve is not a single piece")]
    SpiroBroken,
}

pub type Result<T> = std::result::Result<T, Error>;
