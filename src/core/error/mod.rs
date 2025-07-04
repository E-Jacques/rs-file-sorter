pub mod strategy_validator_error;

#[derive(Debug)]
pub enum Error {
    Validation(String, strategy_validator_error::Error),
    Strategy(String),
    IO(std::io::Error),
    Pipeline,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Validation(strategy_name, err) => write!(f, "{strategy_name}: {err}"),
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
            Error::Validation(_, e) => Some(e),
            Error::IO(e) => Some(e),
            Error::Strategy(_) => None,
            Error::Pipeline => None,
        }
    }
}
