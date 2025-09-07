#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    InvalidClusterNumber,
    InvalidIterations,
    NotEnoughPoints,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorKind {
    InvalidClusterNumber,
    InvalidIterations,
    NotEnoughPoints,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::InvalidClusterNumber => ErrorKind::InvalidClusterNumber,
            Error::InvalidIterations => ErrorKind::InvalidIterations,
            Error::NotEnoughPoints => ErrorKind::NotEnoughPoints,
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::InvalidClusterNumber => "InvaliClusterNumber",
            ErrorKind::InvalidIterations => "InvalidInterations",
            ErrorKind::NotEnoughPoints => "NotEnoughPoints",
        }
        .to_string()
    }
}
