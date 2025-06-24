use crate::utils::logger::Logger;

use super::{cli_handler_builder::ArgValueTypes, parser::ParsedCommand};

#[derive(Debug)]
pub enum Error {
    UnknownArgument,
    UnexpectedValue(ArgValueTypes),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownArgument => write!(f, "Unknown argument provided"),
            Error::UnexpectedValue(value) => write!(f, "Unexpected value: {:?}", value),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CommandBuilder {
    pub name: String,
    pub description: String,
    pub logger: Logger,
}

#[derive(Debug, Clone)]
pub struct ArgBuilder {
    pub name: String,
    pub description: String,
    pub expected_value_type: Vec<ArgValueTypes>,
    pub parent_name: Option<String>,
}

impl ArgBuilder {
    pub fn validate(&self, parsed_command: ParsedCommand) -> Result<(), Error> {
        let parsed_arg = match parsed_command
            .args
            .into_iter()
            .find(|arg| arg.arg_name == self.name.to_string())
        {
            Some(parsed_arg) => parsed_arg,
            None => {
                return Err(Error::UnknownArgument);
            }
        };

        // FIXME: add recursive validation
        if self.parent_name.is_some() {
            return Ok(());
        }

        let parsed_arg_type_accepted = self.expected_value_type.iter().any(|expected_value| {
            ArgValueTypes::from(parsed_arg.arg_value.clone()) == *expected_value
        });
        if !parsed_arg_type_accepted {
            return Err(Error::UnexpectedValue(ArgValueTypes::from(
                parsed_arg.arg_value,
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ParamBuilder {
    pub name: String,
    pub description: String,
}
