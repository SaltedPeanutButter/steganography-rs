use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    SysIOError(std::io::Error),
    PngDecodingError(png::DecodingError),
    PngEncodingError(png::EncodingError),

    DataTooLarge(usize, usize),
    HashMismatch(u64, u64),

    TransformationError(Box<dyn Error>),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SysIOError(_) => write!(f, "failed to perform I/O by system"),
            Self::PngDecodingError(_) => write!(f, "failed to decode PNG image"),
            Self::PngEncodingError(_) => write!(f, "failed to encode PNG image"),
            Self::DataTooLarge(actual, available) => write!(
                f,
                "cannot fit {} bits in {} bytes available",
                actual, available
            ),
            Self::HashMismatch(expected, actual) => {
                write!(f, "expecting hash '{}', found '{}'", expected, actual)
            }
            Self::TransformationError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SysIOError(e) => Some(e),
            Self::PngDecodingError(e) => Some(e),
            Self::PngEncodingError(e) => Some(e),
            Self::DataTooLarge(_, _) | Self::HashMismatch(_, _) => None,
            Self::TransformationError(e) => Some(e.as_ref()),
        }
    }
}
