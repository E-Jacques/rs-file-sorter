use super::cli_handler_builder::ArgValueTypes;

#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue {
    NotProvided,
    NoValue,
    Multiple(Vec<String>),
    Single(String),
}

impl ArgValue {
    pub fn is_same_type(&self, compare_to: ArgValueTypes) -> bool {
        match self {
            ArgValue::NotProvided => true,
            ArgValue::NoValue => compare_to == ArgValueTypes::NoValue,
            ArgValue::Multiple(_) => compare_to == ArgValueTypes::Multiple,
            ArgValue::Single(_) => compare_to == ArgValueTypes::Single,
        }
    }
}

impl std::fmt::Display for ArgValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgValue::NotProvided => write!(f, "not provided"),
            ArgValue::NoValue => write!(f, "alone"),
            ArgValue::Multiple(_) => write!(f, "multiples values"),
            ArgValue::Single(_) => write!(f, "only one value"),
        }
    }
}

impl From<ArgValue> for String {
    fn from(value: ArgValue) -> Self {
        match value {
            ArgValue::NotProvided => String::from("NotProvided"),
            ArgValue::NoValue => String::from("NoValue"),
            ArgValue::Multiple(_) => String::from("Multiple"),
            ArgValue::Single(_) => String::from("Single"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedArgs {
    pub arg_name: String,
    pub arg_value: ArgValue,
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub command_name: String,
    pub args: Vec<ParsedArgs>,
    pub params: Vec<String>,
}

pub fn parse_cli(
    command: Vec<String>,
    expected_args: Vec<String>,
    arg_prefix: String,
) -> ParsedCommand {
    let mut parsed_command = parse_parameters_and_args(&command, arg_prefix);
    parsed_command = group_args_by_name(&parsed_command);
    parsed_command = add_not_provided_but_expected_args(&parsed_command, expected_args);

    parsed_command
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
                arg_value: ArgValue::NoValue,
            });
            continue;
        }

        if current_arg.is_none() {
            params.push(String::from(value));
        } else {
            if let Some(mut current) = current_arg {
                current.arg_value = ArgValue::Single(String::from(value));
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

fn group_args_by_name(parsed_command: &ParsedCommand) -> ParsedCommand {
    let mut grouped_args: Vec<ParsedArgs> = vec![];
    for arg in &parsed_command.args {
        let current_arg_name = arg.arg_name.clone();
        let arg_value = arg.arg_value.clone();
        match grouped_args
            .iter_mut()
            .find(|a| a.arg_name == current_arg_name.clone())
        {
            Some(argument) => {
                if let ArgValue::Single(value) = arg_value {
                    argument.arg_value = match &argument.arg_value {
                        ArgValue::NotProvided => ArgValue::Single(String::from(value)),
                        ArgValue::Multiple(a) => {
                            let mut vec_clone = a.clone();
                            vec_clone.push(String::from(value));
                            ArgValue::Multiple(vec_clone)
                        }
                        ArgValue::Single(a) => {
                            ArgValue::Multiple(vec![a.clone(), String::from(value)])
                        }
                        ArgValue::NoValue => ArgValue::Single(String::from(value)),
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

    ParsedCommand {
        command_name: parsed_command.command_name.clone(),
        args: grouped_args,
        params: parsed_command.params.clone(),
    }
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
