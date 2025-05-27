use super::{cli_handler::CliHandlerCommand, cli_handler_builder::ArgValueTypes};

#[derive(Debug, PartialEq, Clone)]
pub struct ArgDatum {
    pub value: Option<String>,
    pub child_args: Vec<ParsedArgs>,
}

impl ArgDatum {
    pub fn new() -> ArgDatum {
        ArgDatum {
            value: None,
            child_args: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue {
    NotProvided,
    Multiple(Vec<ArgDatum>),
    Single(ArgDatum),
}

#[derive(Debug)]
pub enum ParserError {
    UnkownArgument(ParsedArgs),
}

impl From<ArgValue> for ArgValueTypes {
    fn from(value: ArgValue) -> Self {
        match value {
            ArgValue::NotProvided => ArgValueTypes::NoValue,
            ArgValue::Multiple(_) => ArgValueTypes::Multiple,
            ArgValue::Single(value) => {
                if value.value.is_some() {
                    ArgValueTypes::Single
                } else {
                    ArgValueTypes::NoValue
                }
            }
        }
    }
}

impl ArgValue {
    pub fn is_same_type(&self, compare_to: ArgValueTypes) -> bool {
        match self {
            ArgValue::NotProvided => true,
            ArgValue::Multiple(_) => compare_to == ArgValueTypes::Multiple,
            ArgValue::Single(value) => {
                if value.value.is_some() {
                    compare_to == ArgValueTypes::Single
                } else {
                    compare_to == ArgValueTypes::NoValue
                }
            }
        }
    }
}

impl std::fmt::Display for ArgValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgValue::NotProvided => write!(f, "not provided"),
            ArgValue::Multiple(_) => write!(f, "multiples values"),
            ArgValue::Single(_) => write!(f, "only one value"),
        }
    }
}

impl From<ArgValue> for String {
    fn from(value: ArgValue) -> Self {
        match value {
            ArgValue::NotProvided => String::from("NotProvided"),
            ArgValue::Multiple(_) => String::from("Multiple"),
            ArgValue::Single(_) => String::from("Single"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedArgs {
    pub arg_name: String,
    pub arg_value: ArgValue,
}

impl From<&mut ParsedArgs> for ParsedArgs {
    fn from(value: &mut ParsedArgs) -> Self {
        ParsedArgs {
            arg_name: value.arg_name.clone(),
            arg_value: value.arg_value.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub command_name: String,
    pub args: Vec<ParsedArgs>,
    pub params: Vec<String>,
}

pub fn parse_cli(
    command: Vec<String>,
    command_handler: &CliHandlerCommand,
    arg_prefix: String,
) -> Result<ParsedCommand, ParserError> {
    let mut parsed_command = parse_parameters_and_args(&command, arg_prefix);
    println!("parse : {:?}", parsed_command);
    match handle_nested_arguments(command_handler, parsed_command.args) {
        Ok(arguments) => {
            parsed_command.args = arguments;
        }
        Err(parser_error) => {
            return Err(parser_error);
        }
    }
    println!("parse post nested : {:?}", parsed_command);
    parsed_command.args = group_args_by_name(parsed_command.args);
    println!("parse mid : {:?}", parsed_command);

    let expected_args: Vec<String> = command_handler
        .args
        .iter()
        .map(|arg| arg.name.clone())
        .collect();
    parsed_command = add_not_provided_but_expected_args(&parsed_command, expected_args);
    println!("parse end : {:?}", parsed_command);

    Ok(parsed_command)
}

fn parse_parameters_and_args(command: &Vec<String>, arg_prefix: String) -> ParsedCommand {
    let command_name = command
        .first()
        .expect("[Cli Parser] Expect to receive non-empty command input.");
    let mut args: Vec<ParsedArgs> = vec![];
    let mut params: Vec<String> = vec![];
    let mut current_arg: Option<ParsedArgs> = None;
    for i in 1..command.len() {
        let value = &command[i];
        if value.starts_with(&arg_prefix) {
            let arg_name = value
                .strip_prefix(&arg_prefix)
                .expect("[Cli Parser] An error occured.");

            if let Some(arg_value) = current_arg {
                args.push(arg_value);
            }

            current_arg = Some(ParsedArgs {
                arg_name: String::from(arg_name),
                arg_value: ArgValue::Single(ArgDatum {
                    value: None,
                    child_args: vec![],
                }),
            });
            continue;
        }

        if current_arg.is_none() {
            params.push(String::from(value));
        } else {
            if let Some(mut current) = current_arg {
                current.arg_value = ArgValue::Single(ArgDatum {
                    value: Some(value.into()),
                    child_args: vec![],
                });
                args.push(current);
            }

            current_arg = None;
        }
    }

    // Handle last argument if its a no value argument.
    if let Some(arg_value) = current_arg {
        args.push(arg_value);
    }

    ParsedCommand {
        command_name: String::from(command_name).clone(),
        args,
        params,
    }
}

fn group_args_by_name(args: Vec<ParsedArgs>) -> Vec<ParsedArgs> {
    let mut grouped_args: Vec<ParsedArgs> = vec![];
    for arg in args {
        let current_arg_name = arg.arg_name.clone();
        let arg_value = arg.arg_value.clone();
        match grouped_args
            .iter_mut()
            .find(|a| a.arg_name == current_arg_name.clone())
        {
            Some(argument) => {
                if let ArgValue::Single(mut value) = arg_value {
                    value.child_args = group_args_by_name(value.child_args);
                    argument.arg_value = match &argument.arg_value {
                        ArgValue::NotProvided => ArgValue::Single(value),
                        ArgValue::Multiple(a) => {
                            let mut vec_clone = a.clone();
                            vec_clone.push(value);
                            ArgValue::Multiple(vec_clone)
                        }
                        ArgValue::Single(a) => ArgValue::Multiple(vec![a.clone(), value]),
                    };
                }
            }
            None => {
                grouped_args.push(ParsedArgs {
                    arg_name: current_arg_name.clone(),
                    arg_value: arg_value.clone(),
                });
            }
        };
    }

    grouped_args
}

fn add_not_provided_but_expected_args(
    parsed_command: &ParsedCommand,
    expected_args: Vec<String>,
) -> ParsedCommand {
    let mut with_args = parsed_command.args.clone();
    for arg_name in expected_args {
        if !parsed_command
            .args
            .iter()
            .any(|arg| arg.arg_name == arg_name)
        {
            with_args.push(ParsedArgs {
                arg_name: String::from(arg_name),
                arg_value: ArgValue::NotProvided,
            });
        }
    }

    ParsedCommand {
        command_name: parsed_command.command_name.clone(),
        args: with_args,
        params: parsed_command.params.clone(),
    }
}

fn handle_nested_arguments(
    command_handler: &CliHandlerCommand,
    args: Vec<ParsedArgs>,
) -> Result<Vec<ParsedArgs>, ParserError> {
    let mut new_args: Vec<ParsedArgs> = vec![];
    for arg in args {
        let maybe_arg_spec = command_handler.args.iter().find(|a| a.name == arg.arg_name);
        match maybe_arg_spec {
            Some(arg_spec) => {
                if let Some(parent_name) = arg_spec.parent_name.clone() {
                    if let Some(parent_arg) = new_args
                        .iter_mut()
                        .rev()
                        .find(|a| a.arg_name.clone() == parent_name)
                    {
                        if let ArgValue::Single(ref mut datum) = parent_arg.arg_value {
                            datum.child_args.push(arg.clone());
                        }
                    }
                } else {
                    new_args.push(arg.clone());
                }
            }
            None => return Err(ParserError::UnkownArgument(arg)),
        }
    }

    Ok(new_args)
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::cli_handler::compound_structs::{ArgBuilder, ParamBuilder},
        utils::logger::Logger,
    };

    use super::*;

    #[test]
    fn test_parse_cli() {
        let command = vec![
            String::from("sort"),
            String::from("from"),
            String::from("to"),
            String::from("--stack"),
            String::from("images"),
            String::from("--sub-stack"),
            String::from("png"),
        ];
        let command_handler = CliHandlerCommand {
            args: vec![
                ArgBuilder {
                    name: String::from("stack"),
                    description: String::from("rule that specify where to put the files. first specified element will be the first directory that you will met and so."),
                    expected_value_type: vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
                    parent_name: None,
                },
                ArgBuilder {
                    name: String::from("sub-stack"),
                    description: String::from("associate a sorting rule with the previous combined stack."),
                    expected_value_type: vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
                    parent_name: Some(String::from("stack")),
                }
            ],
            params: vec![
                ParamBuilder {
                    name: String::from("from"),
                    description: String::from("the directory from which you need to extract the files."),
                },
                ParamBuilder {
                    name: String::from("to"),
                    description: String::from("the directory to which you need to move the files."),
                },
            ],
            logger: Logger::new("Test Logger", false),
            command_name: String::from("sort"),
            command_description: String::from("Sort files."),
            handler: |_, _| {},
        };
        let parsed_command = parse_cli(command, &command_handler, String::from("--")).unwrap();
        assert_eq!(parsed_command.command_name, "sort");
        assert_eq!(parsed_command.params.len(), 2);
        assert_eq!(parsed_command.args.len(), 2);
    }

    mod tests_handle_nested_arguments {
        use crate::cli::cli_handler::compound_structs::ArgBuilder;

        use super::*;

        #[test]
        fn test_handle_nested_arguments() {
            let command_handler = CliHandlerCommand {
                args: vec![
                    ArgBuilder {
                        name: String::from("stack"),
                        description: String::from("rule that specify where to put the files. first specified element will be the first directory that you will met and so."),
                        expected_value_type: vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
                        parent_name: None,
                    },
                    ArgBuilder {
                            name: String::from("sub-stack"),
                            description: String::from("associate a sorting rule with the previous combined stack."),
                            expected_value_type: vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
                            parent_name: Some(String::from("stack")),
                        }
                ],
                params: vec![],
                logger: Logger::new("Test Logger", false),
                command_name: String::from("sort"),
                command_description: String::from("Sort files."),
                handler: |_, _| {},
            };
            let args: Vec<ParsedArgs> = vec![
                ParsedArgs {
                    arg_name: String::from("stack"),
                    arg_value: ArgValue::Single(ArgDatum {
                        value: Some(String::from("images")),
                        child_args: vec![],
                    }),
                },
                ParsedArgs {
                    arg_name: String::from("sub-stack"),
                    arg_value: ArgValue::Single(ArgDatum {
                        value: Some(String::from("png")),
                        child_args: vec![],
                    }),
                },
                ParsedArgs {
                    arg_name: String::from("sub-stack"),
                    arg_value: ArgValue::Single(ArgDatum {
                        value: Some(String::from("jpg")),
                        child_args: vec![],
                    }),
                },
                ParsedArgs {
                    arg_name: String::from("stack"),
                    arg_value: ArgValue::Single(ArgDatum {
                        value: Some(String::from("arg-1")),
                        child_args: vec![],
                    }),
                },
                ParsedArgs {
                    arg_name: String::from("sub-stack"),
                    arg_value: ArgValue::Single(ArgDatum {
                        value: Some(String::from("arg-2")),
                        child_args: vec![],
                    }),
                },
            ];
            let new_args = handle_nested_arguments(&command_handler, args).unwrap();
            assert_eq!(new_args.len(), 2);
            assert_eq!(new_args[0].arg_name, "stack");
            assert_eq!(new_args[1].arg_name, "stack");
            assert_eq!(
                new_args[0].arg_value,
                ArgValue::Single(ArgDatum {
                    value: Some(String::from("images")),
                    child_args: vec![
                        ParsedArgs {
                            arg_name: String::from("sub-stack"),
                            arg_value: ArgValue::Single(ArgDatum {
                                value: Some(String::from("png")),
                                child_args: vec![]
                            })
                        },
                        ParsedArgs {
                            arg_name: String::from("sub-stack"),
                            arg_value: ArgValue::Single(ArgDatum {
                                value: Some(String::from("jpg")),
                                child_args: vec![]
                            })
                        }
                    ]
                })
            );
            assert_eq!(
                new_args[1].arg_value,
                ArgValue::Single(ArgDatum {
                    value: Some(String::from("arg-1")),
                    child_args: vec![ParsedArgs {
                        arg_name: String::from("sub-stack"),
                        arg_value: ArgValue::Single(ArgDatum {
                            value: Some(String::from("arg-2")),
                            child_args: vec![]
                        })
                    }]
                })
            );
        }
    }
}
