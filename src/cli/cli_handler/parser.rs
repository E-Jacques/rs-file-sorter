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

fn init_args(expected_args: Vec<String>) -> Vec<ParsedArgs> {
    let mut args: Vec<ParsedArgs> = vec![];
    for arg_name in expected_args {
        args.push(ParsedArgs {
            arg_name: String::from(arg_name),
            arg_value: ArgValue::NotProvided,
        });
    }
    args
}

pub fn parse_cli(
    command: Vec<String>,
    expected_args: Vec<String>,
    arg_prefix: String,
) -> ParsedCommand {
    let command_name = command
        .first()
        .expect("[Cli Parser] Expect to receive non-empty command input.");

    let mut args = init_args(expected_args.clone());
    let mut params: Vec<String> = vec![];
    let mut current_arg_name = String::from("");

    for i in 1..command.len() {
        let v = &command[i];
        if v.starts_with(&arg_prefix) {
            let arg_name = v
                .strip_prefix(&arg_prefix)
                .expect("[Cli Parser] An error occured.");

            if current_arg_name != "" {
                match args.iter_mut().find(|a| a.arg_name == current_arg_name) {
                    Some(argument) => argument.arg_value = ArgValue::NoValue,
                    None => {
                        args.push(ParsedArgs {
                            arg_name: current_arg_name,
                            arg_value: ArgValue::NotProvided,
                        });
                    }
                }
            }

            current_arg_name = String::from(arg_name);
            continue;
        }

        if current_arg_name == "" {
            params.push(String::from(v));
        } else {
            let current_arg = match args.iter_mut().find(|a| a.arg_name == current_arg_name.clone()) {
                Some(argument) => argument,
                None => {
                    let parsed_arg = ParsedArgs {
                        arg_name: current_arg_name.clone(),
                        arg_value: ArgValue::NotProvided,
                    };
                    args.push(parsed_arg);
                    args.iter_mut().find(|a| a.arg_name == current_arg_name).expect("The correct argument have just been added previously. Should be able to find it in vector.")
                }
            };

            current_arg.arg_value = match &current_arg.arg_value {
                ArgValue::NotProvided => ArgValue::Single(String::from(v)),
                ArgValue::Multiple(a) => {
                    let mut vec_clone = a.clone();
                    vec_clone.push(String::from(v));
                    ArgValue::Multiple(vec_clone)
                }
                ArgValue::Single(a) => ArgValue::Multiple(vec![a.clone(), String::from(v)]),
                ArgValue::NoValue => panic!(
                    "[Cli Parser] Arguments has already been encoutered as providing no value."
                ),
            };

            current_arg_name = String::from("");
        }
    }

    if current_arg_name != "" {
        match args.iter_mut().find(|a| a.arg_name == current_arg_name) {
            Some(argument) => argument.arg_value = ArgValue::NoValue,
            None => {
                args.push(ParsedArgs {
                    arg_name: current_arg_name,
                    arg_value: ArgValue::NotProvided,
                });
            }
        }
    }

    ParsedCommand {
        command_name: String::from(command_name).clone(),
        args,
        params,
    }
}
