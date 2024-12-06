#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StatusCode {
    Success = 0,
    UnknownUser = 1,
    ErrorConn = 2,
    Error = 3,
}

impl From<::libsubid::Error> for StatusCode {
    fn from(value: ::libsubid::Error) -> Self {
        match value {
            ::libsubid::Error::UnknownUser => Self::UnknownUser,
            ::libsubid::Error::Connection => Self::ErrorConn,
            ::libsubid::Error::General => Self::Error,
        }
    }
}
