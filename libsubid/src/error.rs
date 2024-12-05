#[derive(Clone, Copy, Debug)]
pub enum Error {
    UnknownUser,
    Connection,
    General,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::General => write!(f, ""),
            Self::UnknownUser => write!(f, ""),
            Self::Connection => write!(f, ""),
        }
    }
}

impl core::error::Error for Error {}

pub(crate) type Result<T> = core::result::Result<T, Error>;
