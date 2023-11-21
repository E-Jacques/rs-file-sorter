use crate::utils::logger::Logger;

use super::{
    cli_handler_builder::ArgValueTypes,
    parser::{ArgValue, ParsedCommand},
};

pub enum ArgValidationErrorEnum {
    NoError,
    UnknownArgument,
    UnexpectedValue(ArgValue),
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
}

impl ArgBuilder {
    pub fn validate(&self, parsed_command: ParsedCommand) -> ArgValidationErrorEnum {
        println!("{:#?}", parsed_command);
        let parsed_arg = match parsed_command
            .args
            .into_iter()
            .find(|arg| arg.arg_name == self.name.to_string())
        {
            Some(parsed_arg) => parsed_arg,
            None => {
                return ArgValidationErrorEnum::UnknownArgument;
            }
        };

        let parsed_arg_type_accepted = self
            .expected_value_type
            .iter()
            .any(|expected_value| parsed_arg.arg_value.is_same_type(expected_value.clone()));
        if !parsed_arg_type_accepted {
            return ArgValidationErrorEnum::UnexpectedValue(parsed_arg.arg_value);
        }

        ArgValidationErrorEnum::NoError
    }
}

#[derive(Debug, Clone)]
pub struct ParamBuilder {
    pub name: String,
    pub description: String,
}
