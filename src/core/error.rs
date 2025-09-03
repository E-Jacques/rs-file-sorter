#[derive(Debug)]
pub enum Error {
    Validation(super::validation::error::Error),
    Strategy(String),
    IO(std::io::Error),
    Pipeline,
}

#[cfg(test)]
#[derive(PartialEq, Debug)]
pub enum ErrorKind {
    Validation,
    Strategy,
    IO,
    Pipeline,
}

#[cfg(test)]
impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::Validation(_) => ErrorKind::Validation,
            Error::Strategy(_) => ErrorKind::Strategy,
            Error::IO(_) => ErrorKind::IO,
            Error::Pipeline => ErrorKind::Pipeline,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Validation(err) => write!(f, "Validation Error: {err}"),
            Error::Strategy(message) => write!(f, "Strategy Error: {message}"),
            Error::IO(err) => err.fmt(f),
            Error::Pipeline => write!(
                f,
                "A pipeline error occurred. Please report this error with steps to reproduce."
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Validation(e) => Some(e),
            Error::IO(e) => Some(e),
            Error::Strategy(_) => None,
            Error::Pipeline => None,
        }
    }
}
