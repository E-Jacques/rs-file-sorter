#[derive(Debug)]
pub enum DirectoryType {
    Input,
    Output,
}

impl std::fmt::Display for DirectoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectoryType::Input => write!(f, "input"),
            DirectoryType::Output => write!(f, "output"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    WrongParamNumber(usize),
    DirectoryNotFound(DirectoryType, String),
    NotADirectory(String),
    NoStrategyProvided,
    MissingStrategyName,
    UnknownStrategy(String, String),
    SorterError(crate::core::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            super::error::Error::WrongParamNumber(len) => {
                write!(f, "expected 2 params, got {}.", len)
            }
            super::error::Error::DirectoryNotFound(directory_type, relative_path) => {
                write!(
                    f,
                    "{} directory '{}' don't exists",
                    directory_type, relative_path
                )
            }
            super::error::Error::NotADirectory(relative_path) => {
                write!(f, "'{}' isn't a directory", relative_path)
            }
            super::error::Error::MissingStrategyName => {
                write!(f, "A value needs to be assigned to the stack argument.")
            }
            super::error::Error::NoStrategyProvided => {
                write!(f, "stack argument haven't been provided.")
            }
            super::error::Error::UnknownStrategy(name, all_strategy_names) => write!(
                f,
                "Unexpected stack value. Got '{}', expected one of: {}.",
                name, all_strategy_names
            ),
            super::error::Error::SorterError(err) => err.fmt(f),
        }
    }
}
