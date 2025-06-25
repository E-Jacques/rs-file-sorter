use crate::core::strategy_validator::StrategyValidator;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    MissingMandatoryParameter(StrategyValidator),
    UnknownParameter(String),
    TypeError(StrategyValidator),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingMandatoryParameter(validator) => write!(
                f,
                "Missing mandatory parameter: {} of kind {:?}",
                validator.name, validator.kind
            ),
            Error::UnknownParameter(name) => {
                write!(f, "Unknown parameter: {}", name)
            }
            Error::TypeError(validator) => write!(
                f,
                "Type error for parameter: {} expected kind {:?}",
                validator.name, validator.kind
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
