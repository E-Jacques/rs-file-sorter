use crate::utils::logger::Logger;

use super::{cli_handler_builder::ArgValueTypes, parser::ParsedCommand};

pub enum ArgValidationErrorEnum {
    NoError,
    UnknownArgument,
    UnexpectedValue(ArgValueTypes),
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
    pub fn validate(&self, parsed_command: ParsedCommand) -> ArgValidationErrorEnum {
        println!("validate : {:?}", parsed_command.clone());
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

        // FIXME: add recursive validation
        if self.parent_name.is_some() {
            return ArgValidationErrorEnum::NoError;
        }

        let parsed_arg_type_accepted = self.expected_value_type.iter().any(|expected_value| {
            ArgValueTypes::from(parsed_arg.arg_value.clone()) == *expected_value
        });
        if !parsed_arg_type_accepted {
            return ArgValidationErrorEnum::UnexpectedValue(ArgValueTypes::from(
                parsed_arg.arg_value,
            ));
        }

        ArgValidationErrorEnum::NoError
    }
}

#[derive(Debug, Clone)]
pub struct ParamBuilder {
    pub name: String,
    pub description: String,
}
