use crate::utils::logger::Logger;

use super::{
    compound_structs::{ArgBuilder, ArgValidationErrorEnum, ParamBuilder},
    parser::{parse_cli, ParsedCommand},
};

#[derive(Debug, Clone)]
pub struct CliHandlerCommand {
    pub logger: Logger,
    pub command_name: String,
    pub command_description: String,
    pub args: Vec<ArgBuilder>,
    pub params: Vec<ParamBuilder>,
    pub handler: fn(&ParsedCommand, &Logger) -> (),
}

impl CliHandlerCommand {
    pub fn help(&self) -> String {
        let command_name = &self.command_name;
        let mut help_output = String::from("COMMAND\n");
        help_output.push_str(&format!(
            "\t{command_name}: {} (enter 'help {command_name}' for more informations)\n",
            self.command_description,
        ));
        help_output.push_str("\n");
        help_output.push_str("PARAMETERS\n");
        for param in &self.params {
            help_output.push_str(&format!("\t{}: {}\n", param.name, param.description));
        }
        help_output.push_str("\n");
        help_output.push_str("ARGUMENTS\n");
        for arg in &self.args {
            let expected_value_types_string: String = arg
                .expected_value_type
                .clone()
                .into_iter()
                .map(|value_type| Into::<String>::into(value_type))
                .collect::<Vec<String>>()
                .join(", ");
            help_output.push_str(&format!(
                "\t--{}: {} Accepted values: {}\n",
                arg.name, arg.description, expected_value_types_string
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

        if command_name.is_empty() {
            self.logger.error("please provide a valid command.");
        }

        let command_handler = match self.command_handler_from(command_name) {
            Some(value) => value,
            None => {
                self.logger
                    .error(&format!("'{}' isn't a valid command. Please type 'help' to get the list of valid commands.", command_name));
                return;
            }
        };

        match parse_cli(command, &command_handler, arg_prefix) {
            Ok(parsed_command) => {
                if !self.validate(command_handler.clone(), parsed_command.clone()) {
                    return;
                }

                let command_handler_fn = command_handler.handler;
                command_handler_fn(&parsed_command.clone(), &command_handler.logger)
            }
            Err(parser_error) => match parser_error {
                super::parser::ParserError::UnkownArgument(parsed_args) => {
                    command_handler.logger.error(&format!(
                        "unknown argument: got '{}' when expecting {}. See 'help' to get more informations.",
                        parsed_args.arg_name, command_handler.args.iter().map(|arg_spec| arg_spec.name.clone()).collect::<Vec<String>>().join(", ")
                    ));
                }
            },
        }
    }

    fn handle_help(&self, command: &Vec<String>) -> String {
        match command.get(1) {
            None => self.help(),
            Some(specific_command_name) => match self.command_handler_from(specific_command_name) {
                Some(specific_command_handler) => specific_command_handler.help(),
                None => {
                    self.logger.error(
                        "command invalid. Please type 'help' to get the list of valid commands.",
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

    fn extract_command_from_input(input: String) -> Vec<String> {
        let command: Vec<String> = input
            .split(' ')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        command
    }

    fn help(&self) -> String {
        let mut help_output = String::from("COMMANDS\n");
        for command_handler in &self.command_handlers {
            help_output.push_str(&format!(
                "\t{}: {} (enter 'help {}' for more informations)\n",
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
        if let Some(value) = self.validate_args(&command_specification, &parsed_command) {
            return value;
        }

        if command_specification.params.len() < parsed_command.params.len() {
            command_specification.logger.error(&format!(
                "too much parameters: expected {} parameters but received {}.",
                command_specification.params.len(),
                parsed_command.params.len()
            ));
            return false;
        } else if command_specification.params.len() > parsed_command.params.len() {
            command_specification.logger.error(&format!(
                "not enough parameters: expected {} parameters but received {}.",
                command_specification.params.len(),
                parsed_command.params.len()
            ));
            return false;
        }

        true
    }

    fn validate_args(
        &self,
        command_specification: &CliHandlerCommand,
        parsed_command: &ParsedCommand,
    ) -> Option<bool> {
        let arg_specification_names = command_specification
            .args
            .clone()
            .into_iter()
            .map(|arg| arg.name)
            .collect::<Vec<String>>();

        for arg in command_specification.args.clone().into_iter() {
            match arg.validate(parsed_command.clone()) {
                ArgValidationErrorEnum::NoError => {
                    continue;
                }
                ArgValidationErrorEnum::UnknownArgument => {
                    command_specification.logger.error(&format!(
                        "unknown argument: got '{}' when expecting {}. See 'help' to get more informations.",
                        arg.name, arg_specification_names.join(", ")
                    ));
                    return Some(false);
                }
                ArgValidationErrorEnum::UnexpectedValue(received_value) => {
                    let possible_values = arg.clone().expected_value_type;
                    let possible_values_string = possible_values
                        .into_iter()
                        .map(|val| Into::<String>::into(val.clone()))
                        .collect::<Vec<String>>()
                        .join(", ");
                    let received_value_string = Into::<String>::into(received_value);
                    command_specification.logger.error(&format!(
                        "unexpected argument value for {}: expected {} but received {}.",
                        arg.name, possible_values_string, received_value_string
                    ));
                    return Some(false);
                }
            }
        }

        for arg_name in parsed_command
            .clone()
            .args
            .into_iter()
            .map(|arg| arg.arg_name)
        {
            if !arg_specification_names.contains(&arg_name) {
                command_specification.logger.error(&format!(
                "unknown argument: got '{}' when expecting {}. See 'help' to get more informations.",
                arg_name, arg_specification_names.clone().join(", ")
            ));
                return Some(false);
            }
        }
        None
    }
}

#[cfg(test)]
mod cli_handler_help_tests {
    use crate::{
        cli::cli_handler::cli_handler_builder::{ArgValueTypes, CliHandlerBuilder},
        utils::logger::Logger,
    };

    #[test]
    fn test_correctly_display_help() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("my-command"),
                String::from("my-command's description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::Single],
            )
            .parameter(String::from("params-1"), String::from("desc for params-1"))
            .handler(|_, _| ())
            .command(
                String::from("command-2"),
                String::from("command-2's description"),
                Logger::new("my-command", true),
            )
            .handler(|_, _| ())
            .build();

        let help_output = cli_handler.handle_help(&vec![String::from("help")]);
        let expected_output = String::from("COMMANDS\n\tmy-command: my-command's description (enter 'help my-command' for more informations)\n\tcommand-2: command-2's description (enter 'help command-2' for more informations)\n");

        assert_eq!(help_output, expected_output);
    }

    #[test]
    fn test_correctly_display_help_for_specific_command() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("my-command"),
                String::from("my-command's description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1."),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("desc for arg-2."),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .parameter(String::from("params-1"), String::from("desc for params-1"))
            .handler(|_, _| ())
            .command(
                String::from("command-2"),
                String::from("command-2's description"),
                Logger::new("my-command", true),
            )
            .handler(|_, _| ())
            .build();

        let help_output =
            cli_handler.handle_help(&vec![String::from("help"), String::from("my-command")]);
        let expected_output = String::from("COMMAND\n\tmy-command: my-command's description (enter 'help my-command' for more informations)\n\nPARAMETERS\n\tparams-1: desc for params-1\n\nARGUMENTS\n\t--arg-1: desc for arg-1. Accepted values: NoValue\n\t--arg-2: desc for arg-2. Accepted values: Single, Multiple\n");

        assert_eq!(help_output, expected_output);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] command invalid. Please type 'help' to get the list of valid commands."]
    fn test_help_for_unknown_command() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("my-command"),
                String::from("my-command's description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .parameter(String::from("params-1"), String::from("desc for params-1"))
            .handler(|_, _| ())
            .command(
                String::from("command-2"),
                String::from("command-2's description"),
                Logger::new("my-command", true),
            )
            .handler(|_, _| ())
            .build();

        let _ =
            cli_handler.handle_help(&vec![String::from("help"), String::from("unknown-command")]);
    }
}
