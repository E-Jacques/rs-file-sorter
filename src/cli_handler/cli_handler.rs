use crate::utils::logger::Logger;

use super::{
    compound_structs::{ArgBuilder, ParamBuilder},
    parser::{parse_cli, ParsedCommand},
};

#[derive(Debug, Clone)]
pub struct CliHandlerCommand {
    pub logger: Logger,
    pub command_name: String,
    pub command_description: String,
    pub args: Vec<ArgBuilder>,
    pub params: Vec<ParamBuilder>,
    pub handler: fn(&ParsedCommand) -> (),
}

impl CliHandlerCommand {
    pub fn help(&self) -> String {
        let command_name = &self.command_name;
        let mut help_output = String::from("COMMANDS");
        help_output.push_str(&format!(
            "\t{command_name}: {} (enter 'help {command_name}' for more information)\n",
            self.command_description,
        ));
        help_output.push_str("\n");
        help_output.push_str("PARAMETERS\n");
        for param in &self.params {
            help_output.push_str(&format!("\t{}: {}\n", param.name, param.description));
        }
        help_output.push_str("\n");
        help_output.push_str("ARGUMENTS");
        for arg in &self.args {
            help_output.push_str(&format!(
                "\t--{}: {}. Accepted values: {:#?}\n",
                arg.name, arg.description, arg.expected_value_type
            ));
        }

        help_output
    }
}

pub struct CliHandler {
    pub command_handlers: Vec<CliHandlerCommand>,
    pub logger: Logger,
}

impl CliHandler {
    pub fn handle(&self, input: String) {
        let arg_prefix = String::from("--");
        let command = CliHandler::extract_command_from_input(input);
        let command_name = match command.first() {
            Some(value) => value,
            None => {
                self.logger.error("Please provide a valid command.");
                return;
            }
        };

        // If command is help, we want to provide the associated help message.
        if command_name.to_string() == String::from("help") {
            println!("{}", self.handle_help(&command));
            return;
        }

        let command_handler = match self.command_handler_from(command_name) {
            Some(value) => value,
            None => {
                self.logger
                    .error("Command invalid. Please type help to get the list of valid commands.");
                return;
            }
        };
        let expected_args = CliHandler::expected_arg_from(&command_handler);
        let parsed_command = parse_cli(command, expected_args, arg_prefix);
        if !self.validate(command_handler.clone(), parsed_command.clone()) {
            return;
        }

        let command_handler_fn = command_handler.handler;
        command_handler_fn(&parsed_command.clone())
    }

    fn handle_help(&self, command: &Vec<String>) -> String {
        match command.get(1) {
            None => self.help(),
            Some(specific_command_name) => match self.command_handler_from(specific_command_name) {
                Some(specific_command_handler) => specific_command_handler.help(),
                None => {
                    self.logger.error(
                        "Command invalid. Please type help to get the list of valid commands.",
                    );

                    return String::from("unknown command");
                }
            },
        }
    }

    fn command_handler_from(&self, command_name: &String) -> Option<CliHandlerCommand> {
        self.command_handlers
            .clone()
            .into_iter()
            .find(|handler| handler.command_name == command_name.to_string())
    }

    fn expected_arg_from(command_handler: &CliHandlerCommand) -> Vec<String> {
        let expected_args = command_handler
            .args
            .clone()
            .into_iter()
            .map(|arg| arg.name.to_string())
            .collect();
        expected_args
    }

    fn extract_command_from_input(input: String) -> Vec<String> {
        let command: Vec<String> = input
            .split(' ')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        command
    }

    fn help(&self) -> String {
        let mut help_output = String::from("COMMANDS");
        for command_handler in &self.command_handlers {
            help_output.push_str(&format!(
                "\t{}: {} (enter 'help {}' for more information)\n",
                command_handler.command_name,
                command_handler.command_description,
                command_handler.command_name
            ));
        }

        help_output
    }

    fn validate(
        &self,
        command_specification: CliHandlerCommand,
        parsed_command: ParsedCommand,
    ) -> bool {
        for arg in command_specification.args.clone().into_iter() {
            match arg.validate(parsed_command.clone()) {
                super::compound_structs::ArgValidationErrorEnum::NoError => todo!(),
                super::compound_structs::ArgValidationErrorEnum::UnknownArgument => {
                    let all_argument = command_specification
                        .args
                        .clone()
                        .into_iter()
                        .map(|arg| arg.name)
                        .collect::<Vec<String>>()
                        .join(", ");
                    self.logger.error(&format!(
                        "Unknown argument: expected {} but received {}.",
                        all_argument, arg.name
                    ));
                    return false;
                }
                super::compound_structs::ArgValidationErrorEnum::UnexpectedValue(
                    received_value,
                ) => {
                    let possible_values = arg.clone().expected_value_type;
                    self.logger.error(&format!(
                        "Unexpected argument value: expected {:#?} but received {:#?}.",
                        possible_values, received_value
                    ));
                    return false;
                }
            }
        }

        if command_specification.params.len() < parsed_command.params.len() {
            self.logger.error(&format!(
                "Too much parameters: expected {} but recevied {}.",
                command_specification.params.len(),
                parsed_command.params.len()
            ));
            return false;
        } else if command_specification.params.len() > parsed_command.params.len() {
            self.logger.error(&format!(
                "Not enough parameters: expected {} but recevied {}.",
                command_specification.params.len(),
                parsed_command.params.len()
            ));
            return false;
        }

        true
    }
}
