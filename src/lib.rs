use cli_handler::cli_handler_builder::{ArgValueTypes, CliHandlerBuilder};
use commands::sort_command::exec_sort_command;
use utils::logger::Logger;

pub mod cli_handler;
pub mod commands;
pub mod core;
pub mod sorting_strategies;
pub mod utils;

/**
 * Use to write e2e tests.
 */
pub fn handle(input: String, debug_mode: Option<bool>) {
    let logger = Logger::new("Command Handler", debug_mode.unwrap_or(false));
    let sort_command_logger = Logger::new("Sort Command", debug_mode.unwrap_or(false));
    let command_handler = CliHandlerBuilder::new(logger)
    .command(
        String::from("sort"), 
        String::from("sort your files, from/to given directories, we specific rules."), 
        sort_command_logger
    )
    .args(
        String::from("stack"), 
        String::from("rule that specify where to put the files. first specified element will be the first directory that you will met and so."), 
        vec![ArgValueTypes::Single, ArgValueTypes::Multiple]
    )
    .params(
        String::from("from"), String::from("the directory from which you need to extract the files.")
    )
    .params(
        String::from("to"), 
        String::from("the directory to which you need to put the organized files.")
    )
    .handler(|parsed_command, handler_logger| exec_sort_command(parsed_command.args.clone(), parsed_command.params.clone(), handler_logger.clone()))
    .build();

    command_handler.handle(input);
}
