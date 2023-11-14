use crate::{
    sortering_strategies::{sorter, sorting_strategy_list::get_storting_strategies_list},
    utils::file_manipulator::to_absolute_path, cli_handler::parser::{ParsedArgs, ArgValue},
};

pub fn exec_sort_command(args: Vec<ParsedArgs>, params: Vec<String>) {
    if params.len() != 2 {
        panic!("[Sort Command] Expected 2 params, got {}.", params.len());
    }

    let input_dir = &to_absolute_path(params[0].clone());
    let output_dir = &to_absolute_path(params[1].clone());

    let sorting_strategies_list = &get_storting_strategies_list();

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
        ArgValue::NotProvided => panic!("[Sort Command] stack argument haven't been provided."),
        ArgValue::NoValue => {
            panic!("[Sort Command] a value needs to be assigned to the stack argument.")
        }
        ArgValue::Multiple(stacks) => stacks,
        ArgValue::Single(stack) => vec![stack],
    };
    let sorting_strategies = stacks
        .into_iter()
        .map(|stack| {
            sorting_strategies_list
                .into_iter()
                .find(|value| value.name.clone() == stack)
                .expect(&format!(
                    "[Sort Command] Unexpected stack value. Got {}, expected {}.",
                    stack,
                    sorting_strategies_list
                        .into_iter()
                        .map(|v| v.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
        })
        .collect();

    sorter::sorter(
        input_dir,
        output_dir,
        sorting_strategies,
        |old_filename, new_filename| {
            panic!(
                "[Sort Command] Unable to copy file {} to {}.",
                old_filename, new_filename
            );
        },
    )
}

#[cfg(test)]
mod tests_exec_sort_command_panics {
    use super::*;

    #[test]
    #[should_panic(expected = "[Sort Command] Expected 2 params, got 1.")]
    fn test_exec_sort_command_params_length_not_enought() {
        exec_sort_command(vec![], vec![String::from("param-1")]);
    }

    #[test]
    #[should_panic(expected = "[Sort Command] Expected 2 params, got 3.")]
    fn test_exec_sort_command_params_length_too_many() {
        exec_sort_command(
            vec![],
            vec![
                String::from("param-1"),
                String::from("param-2"),
                String::from("param-3"),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "[Sort Command] stack argument haven't been provided.")]
    fn test_exec_sort_command_missing_stack_argument() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::NotProvided,
            }],
            vec![String::from("param-1"), String::from("param-2")],
        );
    }

    #[test]
    #[should_panic(expected = "[Sort Command] a value needs to be assigned to the stack argument.")]
    fn test_exec_sort_command_stack_argument_has_no_value() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::NoValue,
            }],
            vec![String::from("param-1"), String::from("param-2")],
        );
    }

    #[test]
    #[should_panic(
        expected = "[Sort Command] Unexpected stack value. Got unknown_stack, expected month, year."
    )]
    fn test_exec_sort_command_unexpected_stack_value() {
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::Single(String::from("unknown_stack")),
            }],
            vec![String::from("param-1"), String::from("param-2")],
        );
    }
}
