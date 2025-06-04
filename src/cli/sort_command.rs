use std::fs::metadata;

use rsft_utils::common::file_or_dir_exists;

use crate::{
    cli::cli_handler::parser::ArgDatum,
    core::{
        sorter::sorter, sorting_strategy::SortingStrategy, strategy_parameter::StrategyParameter,
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

pub fn exec_sort_command(args: Vec<ParsedArgs>, params: Vec<String>, logger: Logger) {
    if params.len() != 2 {
        logger.error(&format!("expected 2 params, got {}.", params.len()));
    }

    let input_dir = &to_absolute_path(params[0].clone());
    let output_dir = &to_absolute_path(params[1].clone());

    if !file_or_dir_exists(input_dir.clone().into()) {
        logger.error(&format!(
            "input directory '{}' don't exists",
            // We want to replace \\ with / to uniform the output across the OS for tests.
            &to_relative_path(input_dir.clone()).replace("\\", "/")
        ))
    }

    if !file_or_dir_exists(output_dir.clone().into()) {
        logger.error(&format!(
            "output directory '{}' don't exists",
            // We want to replace \\ with / to uniform the output across the OS for tests.
            &to_relative_path(output_dir.clone()).replace("\\", "/")
        ))
    }

    if !metadata(input_dir.clone()).unwrap().is_dir() {
        logger.error(&format!(
            "'{}' isn't a directory",
            // We want to replace \\ with / to uniform the output across the OS for tests.
            &to_relative_path(input_dir.clone()).replace("\\", "/")
        ))
    }

    if !metadata(output_dir.clone()).unwrap().is_dir() {
        logger.error(&format!(
            "'{}' isn't a directory",
            // We want to replace \\ with / to uniform the output across the OS for tests.
            &to_relative_path(output_dir.clone()).replace("\\", "/")
        ))
    }

    let sorting_strategies_list: StrategyCatalog =
        get_metadata_catalog().with(&get_manipulation_catalog());

    let stack_arg_name = String::from("stack");
    let default_stack_parsed_args = ParsedArgs {
        arg_name: stack_arg_name.clone(),
        arg_value: ArgValue::NotProvided,
    };
    let sorting_stacks = args
        .into_iter()
        .find(|arg| arg.arg_name == stack_arg_name)
        .unwrap_or(default_stack_parsed_args);

    let stacks = match sorting_stacks.arg_value {
        ArgValue::NotProvided => {
            logger.error("stack argument haven't been provided.");
            panic!();
        }
        ArgValue::Multiple(stacks) => stacks,
        ArgValue::Single(stack) => vec![stack],
    };

    let sorting_strategies = get_storting_strategies(stacks, sorting_strategies_list, &logger);

    let logger_borrowed = &logger.clone();
    let rename_error_handler = |old_filename: &_, new_filename: &_| {
        logger_borrowed.error(&format!(
            "unable to copy file {} to {}.",
            old_filename, new_filename
        ));
    };

    sorter(
        input_dir,
        output_dir,
        sorting_strategies,
        logger.clone(),
        &rename_error_handler.clone(),
    )
}
fn get_storting_strategies(
    stacks: Vec<ArgDatum>,
    strategy_catalog: StrategyCatalog,
    logger: &Logger,
) -> Vec<SortingStrategy> {
    let all_strategy_names = strategy_catalog.get_names().join(", ");

    stacks
        .into_iter()
        .map(|arg_datum| {
            let name = arg_datum.value.as_ref().unwrap_or_else(|| {
                logger.error("A value needs to be assigned to the stack argument.");
                panic!();
            });

            let mut strategy = strategy_catalog.get_strategy(name).unwrap_or_else(|| {
                logger.error(&format!(
                    "Unexpected stack value. Got '{}', expected one of: {}.",
                    name, all_strategy_names
                ));
                panic!();
            });

            if let Some(parsed_arg) = arg_datum
                .child_args
                .iter()
                .find(|arg| arg.arg_name == "strategies")
            {
                let child_strategies = match &parsed_arg.arg_value {
                    ArgValue::NotProvided => vec![],
                    ArgValue::Multiple(datums) => datums.clone(),
                    ArgValue::Single(datum) => vec![datum.clone()],
                }
                .into_iter()
                .filter_map(|datum| {
                    datum
                        .value
                        .as_ref()
                        .and_then(|name| strategy_catalog.get_strategy(name))
                        .map(|s| Box::new(s.clone()))
                })
                .collect();

                strategy.add_parameter(
                    "strategies".to_string(),
                    StrategyParameter::Strategy(child_strategies),
                );
            }

            strategy
        })
        .collect()
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
        expected = "[ERROR] [Sort Command] Unexpected stack value. Got 'unknown_stack', expected one of: month, year, concat, text."
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
