use std::{collections::HashMap, fs::metadata};

use rsft_utils::common::file_or_dir_exists;

use crate::{
    cli::cli_handler::parser::ArgDatum,
    core::{
        parameter::{StrategyParameter, StrategyParameterKind},
        strategy::Strategy,
        validation,
    },
    sorting_strategies::{
        manipulation_catalog::get_manipulation_catalog, metadata_catalog::get_metadata_catalog,
        strategy_catalog::StrategyCatalog,
    },
    utils::{
        file_manipulator::{to_absolute_path, to_relative_path},
        logger::Logger,
    },
};

use super::cli_handler::parser::{ArgValue, ParsedArgs};

pub static PARAMETER: &str = "parameter";
pub static DRY_RUN: &str = "dry-run";
pub static ROOT_ONLY: &str = "root-only";
pub static STACK: &str = "stack";

static PARAMETER_SEP: &'static str = "=";

impl std::fmt::Display for crate::core::report::Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.result {
            Ok(target) => {
                write!(
                    f,
                    "{} -> {}",
                    self.input_filename.display(),
                    target.display()
                )
            }
            Err(err) => {
                write!(f, "{} -x {}", self.input_filename.display(), err)
            }
        }
    }
}

fn get_strategy_validator(
    strategy: Box<dyn Strategy>,
    name: &String,
) -> Option<validation::ParameterDetail> {
    strategy
        .parameter_details()
        .into_iter()
        .find(|detail| detail.name == *name)
}

pub fn exec_sort_command(args: Vec<ParsedArgs>, params: Vec<String>, logger: Logger) {
    let sorting_strategies_list: StrategyCatalog =
        get_metadata_catalog().with(&get_manipulation_catalog());

    let dry_run: bool = get_bool_arg_value(&args, DRY_RUN);
    let root_level_only: bool = get_bool_arg_value(&args, ROOT_ONLY);

    match get_cli_inputs(args, params, STACK, sorting_strategies_list).and_then(
        |(input_dir, output_dir, sorting_strategies)| {
            crate::core::SortPipeline::new(
                input_dir,
                output_dir,
                sorting_strategies,
                crate::core::options::SortOptions {
                    dry_run,
                    root_level_only,
                },
            )
            .process()
            .map_err(super::error::Error::SorterError)
        },
    ) {
        Err(err) => handle_errors(&logger, err),
        Ok(reports) if dry_run => {
            for report in reports.unwrap_or_default() {
                println!("{}", report)
            }
        }
        _ => (),
    };
}

fn get_bool_arg_value(args: &Vec<ParsedArgs>, arg_name: &str) -> bool {
    args.iter()
        .any(|a| a.arg_name == arg_name.to_string() && a.arg_value != ArgValue::NotProvided)
}

fn get_cli_inputs(
    args: Vec<ParsedArgs>,
    params: Vec<String>,
    stack_arg_name: &str,
    sorting_strategies_list: StrategyCatalog,
) -> Result<(String, String, Vec<Box<dyn Strategy>>), super::error::Error> {
    let (input, output) = get_directories(params)?;
    let stacks = get_stacks(args, stack_arg_name)?;
    let sorting_strategies = get_storting_strategies(stacks, sorting_strategies_list)?;

    Ok((input, output, sorting_strategies))
}

fn get_stacks(
    args: Vec<ParsedArgs>,
    stack_arg_name: &str,
) -> Result<Vec<ArgDatum>, super::error::Error> {
    match args.into_iter().find(|arg| arg.arg_name == stack_arg_name) {
        Some(sorting_stacks) => match sorting_stacks.arg_value {
            ArgValue::NotProvided => Err(super::error::Error::NoStrategyProvided),
            ArgValue::Multiple(stacks) => Ok(stacks),
            ArgValue::Single(stack) => Ok(vec![stack]),
        },
        None => Err(super::error::Error::NoStrategyProvided),
    }
}

fn handle_errors(logger: &Logger, err: super::error::Error) {
    logger.error(&err.to_string());
}

fn get_directories(params: Vec<String>) -> Result<(String, String), super::error::Error> {
    fn validate_directory(
        path: &String,
        dir_type: super::error::DirectoryType,
    ) -> Result<(), super::error::Error> {
        let relative_path = to_relative_path(path.clone());
        if !file_or_dir_exists(path.into()) {
            Err(super::error::Error::DirectoryNotFound(
                dir_type,
                relative_path,
            ))
        } else if !metadata(path).map(|m| m.is_dir()).unwrap_or(false) {
            Err(super::error::Error::NotADirectory(relative_path))
        } else {
            Ok(())
        }
    }

    if params.len() != 2 {
        return Err(super::error::Error::WrongParamNumber(params.len()));
    }

    let input_dir = to_absolute_path(params[0].clone());
    validate_directory(&input_dir, super::error::DirectoryType::Input)?;

    let output_dir = to_absolute_path(params[1].clone());
    validate_directory(&output_dir, super::error::DirectoryType::Output)?;

    Ok((input_dir, output_dir))
}

fn get_storting_strategies(
    stacks: Vec<ArgDatum>,
    strategy_catalog: StrategyCatalog,
) -> Result<Vec<Box<dyn Strategy>>, super::error::Error> {
    let all_strategy_names = strategy_catalog.get_names().join(", ");

    stacks
        .into_iter()
        .map(|arg_datum| {
            let name = arg_datum
                .value
                .clone()
                .ok_or(super::error::Error::MissingStrategyName)?;

            let mut strategy = strategy_catalog.get_strategy(&name).ok_or(
                super::error::Error::UnknownStrategy(name.clone(), all_strategy_names.clone()),
            )?;

            if let Some(parsed_arg) = arg_datum.get_child(PARAMETER) {
                for (key, value) in &get_parameters_from_parsed_args(parsed_arg) {
                    if let Some(parameter_value) = get_strategy_validator(strategy.clone(), key)
                        .and_then(|validator| {
                            get_parameter_value(&strategy_catalog, &validator, value)
                        })
                    {
                        strategy.add_parameter(key.clone(), parameter_value);
                    }
                }
            }

            Ok(strategy)
        })
        .collect()
}

fn get_parameter_value(
    strategy_catalog: &StrategyCatalog,
    validator: &validation::ParameterDetail,
    value: &Vec<String>,
) -> Option<StrategyParameter> {
    match validator.kind {
        StrategyParameterKind::Strategy => Some(StrategyParameter::Strategy(
            value
                .iter()
                .map(|v| strategy_catalog.get_strategy(v))
                .flatten()
                .collect(),
        )),
        StrategyParameterKind::Choice(_) | StrategyParameterKind::SingleString => value
            .iter()
            .last()
            .cloned()
            .map(StrategyParameter::SingleString),
        StrategyParameterKind::Number => value
            .iter()
            .last()
            .and_then(|v| v.parse::<usize>().ok())
            .map(StrategyParameter::Number),
    }
}

fn get_parameters_from_parsed_args(parsed_arg: &ParsedArgs) -> HashMap<String, Vec<String>> {
    let child_parameters = match &parsed_arg.arg_value {
        ArgValue::NotProvided => vec![],
        ArgValue::Multiple(datums) => datums.clone(),
        ArgValue::Single(datum) => vec![datum.clone()],
    };
    let mut parameters: HashMap<String, Vec<String>> = HashMap::new();

    child_parameters.iter().for_each(|param| {
        if let Some(value) = &param.value {
            let keyvalue = value.split(PARAMETER_SEP).collect::<Vec<&str>>();
            let key = keyvalue[0].to_string();
            let value = keyvalue[1].to_string();

            match parameters.get_mut(&key.clone()) {
                Some(param_mut) => {
                    param_mut.push(value);
                }
                None => {
                    parameters.insert(key, vec![value]);
                }
            }
        }
    });
    parameters
}

#[cfg(test)]
mod tests_exec_sort_command_panics {
    use crate::cli::cli_handler::parser::ArgDatum;

    use super::*;

    #[test]
    #[should_panic(expected = "[ERROR] [Sort Command] expected 2 params, got 1.")]
    fn test_exec_sort_command_params_length_not_enought() {
        exec_sort_command(
            vec![],
            vec![String::from("tests/rsc/files/output/")],
            Logger::new("Sort Command", true),
        );
    }

    #[test]
    #[should_panic(expected = "[ERROR] [Sort Command] expected 2 params, got 3.")]
    fn test_exec_sort_command_params_length_too_many() {
        exec_sort_command(
            vec![],
            vec![
                String::from("tests/rsc/files/output/"),
                String::from("tests/rsc/files/output/"),
                String::from("tests/rsc/files/output/"),
            ],
            Logger::new("Sort Command", true),
        );
    }

    #[test]
    #[should_panic(expected = "[ERROR] [Sort Command] stack argument haven't been provided.")]
    fn test_exec_sort_command_missing_stack_argument() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::NotProvided,
            }],
            vec![
                String::from("tests/rsc/files/output/"),
                String::from("tests/rsc/files/output/"),
            ],
            Logger::new("Sort Command", true),
        );
    }

    #[test]
    #[should_panic(
        expected = "[ERROR] [Sort Command] A value needs to be assigned to the stack argument."
    )]
    fn test_exec_sort_command_stack_argument_has_no_value() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::Single(ArgDatum::new()),
            }],
            vec![
                String::from("tests/rsc/files/output/"),
                String::from("tests/rsc/files/output/"),
            ],
            Logger::new("Sort Command", true),
        );
    }

    #[test]
    #[should_panic(
        expected = "[ERROR] [Sort Command] Unexpected stack value. Got 'unknown_stack', expected one of: month, year, file extension, file type, concat, text, or."
    )]
    fn test_exec_sort_command_unexpected_stack_value() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::Single(ArgDatum {
                    value: Some(String::from("unknown_stack")),
                    child_args: vec![],
                }),
            }],
            vec![
                String::from("tests/rsc/files/output/"),
                String::from("tests/rsc/files/output/"),
            ],
            Logger::new("Sort Command", true),
        );
    }
}
