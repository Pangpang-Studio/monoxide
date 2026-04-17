pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("not enough spiro control points to create bezier path")]
    NotEnoughCps,
}
