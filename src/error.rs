pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Window not found")]
    WindowNotFound,
    #[error(transparent)]
    GifDecoding(#[from] gif::DecodingError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
