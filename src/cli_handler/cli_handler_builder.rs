use crate::utils::logger::Logger;

use super::{
    cli_handler::{CliHandler, CliHandlerCommand},
    compound_structs::{ArgBuilder, CommandBuilder, ParamBuilder},
    parser::ParsedCommand,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgValueTypes {
    NoValue,
    Multiple,
    Single,
}

pub struct CliHandlerBuilder {
    current_command: Option<CommandBuilder>,
    current_args: Option<Vec<ArgBuilder>>,
    current_params: Option<Vec<ParamBuilder>>,
    current_handler: Option<fn(&ParsedCommand) -> ()>,
    commands: Vec<CliHandlerCommand>,
    logger: Logger,
}

impl CliHandlerBuilder {
    pub fn new(logger: Logger) -> Self {
        CliHandlerBuilder {
            current_command: None,
            current_args: None,
            current_params: None,
            current_handler: None,
            commands: vec![],
            logger,
        }
    }

    pub fn command(mut self, name: String, description: String, logger: Logger) -> Self {
        // Error case isn't problematic in command method.
        // It only means that we're initiating the first command specification.
        let _ = self.push_current_command();

        self.current_args = None;
        self.current_params = None;
        self.current_command = Some(CommandBuilder {
            name,
            description,
            logger: logger.clone(),
        });

        self
    }

    pub fn args(
        mut self,
        name: String,
        description: String,
        expected_value_type: Vec<ArgValueTypes>,
    ) -> Self {
        let arg_builder = ArgBuilder {
            name: name.clone(),
            description,
            expected_value_type: expected_value_type.clone(),
        };

        if expected_value_type.is_empty() {
            self.logger
                .error("args method cannot receive empty 'expected_value_type' vector.");
        }

        if self.current_command.is_none() {
            self.logger.error(&format!(
                "cannot specify argument '{name}' outside of command context."
            ));
        }

        match &self.current_args {
            Some(arg) => {
                let arg_already_specified = arg.iter().any(|a| a.name == name);
                if arg_already_specified {
                    let command_name = self
                        .current_command
                        .clone()
                        .expect("Should be able to acces command when calling args")
                        .name
                        .clone();
                    self.logger.error(&format!(
                        "argument '{name}' have already been declared for command '{}'.",
                        command_name
                    ))
                }
                let mut arg_vec = arg.clone();

                arg_vec.push(arg_builder);
                self.current_args = Some(arg_vec);
            }
            None => {
                self.current_args = Some(vec![arg_builder]);
            }
        };

        self
    }

    pub fn params(mut self, name: String, description: String) -> Self {
        let param_builder = ParamBuilder {
            name: name.clone(),
            description,
        };

        if self.current_command.is_none() {
            self.logger.error(&format!(
                "cannot specify parameter '{name}' outside of command context."
            ));
        }

        match &self.current_params {
            Some(param) => {
                let param_already_specified = param.iter().any(|a| a.name == name);
                if param_already_specified {
                    let command_name = self
                        .current_command
                        .clone()
                        .expect("Should be able to acces command when calling args")
                        .name
                        .clone();
                    self.logger.error(&format!(
                        "parameter '{name}' have already been declared for command '{}'.",
                        command_name
                    ))
                }
                let mut param_vec = param.clone();
                param_vec.push(param_builder);
                self.current_params = Some(param_vec);
            }
            None => {
                self.current_params = Some(vec![param_builder]);
            }
        };

        self
    }

    pub fn handler(mut self, handler: fn(&ParsedCommand) -> ()) -> Self {
        match self.current_handler {
            Some(_) => self
                .logger
                .error("handler should only be set once per command."),
            None => self.current_handler = Some(handler),
        };

        self
    }

    pub fn build(mut self) -> CliHandler {
        match self.push_current_command() {
            Err(_) => self.logger.error("no command has been specified."),
            Ok(_) => (),
        };

        CliHandler {
            command_handlers: self.commands.clone(),
            logger: self.logger.clone(),
        }
    }

    fn push_current_command(&mut self) -> Result<(), ()> {
        match &self.current_command {
            None => Err(()),
            Some(current_command) => {
                if self.current_handler.is_none() {
                    self.logger
                        .error("command specificatino needs to have a handler closure associated!");
                }

                self.commands.push(CliHandlerCommand {
                    command_name: current_command.name.to_string(),
                    command_description: current_command.description.to_string(),
                    args: self.current_args.take().unwrap_or(vec![]).to_vec(),
                    params: self.current_params.take().unwrap_or(vec![]).to_vec(),
                    logger: current_command.logger.clone(),
                    handler: self.current_handler.unwrap(),
                });

                Ok(())
            }
        }
    }
}
