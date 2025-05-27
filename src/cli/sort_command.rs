use std::fs::metadata;

use rsft_utils::common::file_or_dir_exists;

use crate::{
    core::{sorter::sorter, sorting_strategy::SortingStrategy},
    sorting_strategies::{get_month_sorting_strategy, get_year_sorting_strategy},
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

    let sorting_strategies_list = vec![get_month_sorting_strategy(), get_year_sorting_strategy()];

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

    let stack_names: Vec<String> = stacks
        .into_iter()
        .map(|arg| match arg.value {
            Some(stack_name) => stack_name,
            None => {
                logger.error("a value needs to be assigned to the stack argument.");
                panic!();
            }
        })
        .collect();
    let sorting_strategies = get_storting_strategies(stack_names, sorting_strategies_list, &logger);

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
    stacks: Vec<String>,
    sorting_strategies_list: Vec<SortingStrategy>,
    logger: &Logger,
) -> Vec<SortingStrategy> {
    let all_sorting_strategies_string = sorting_strategies_list
        .iter()
        .map(|v| v.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    let sorting_strategies = stacks
        .into_iter()
        .map(|stack| {
            match sorting_strategies_list
                .iter()
                .find(|value| value.name == stack)
            {
                Some(output) => output.clone(),
                None => {
                    logger.error(&format!(
                        "unexpected stack value. Got {}, expected {}.",
                        stack,
                        all_sorting_strategies_string.clone()
                    ));
                    panic!()
                }
            }
        })
        .collect();
    sorting_strategies
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
        expected = "[ERROR] [Sort Command] a value needs to be assigned to the stack argument."
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
        expected = "[ERROR] [Sort Command] unexpected stack value. Got unknown_stack, expected month, year."
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
