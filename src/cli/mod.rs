use cli_handler::cli_handler_builder::{ArgValueTypes, CliHandlerBuilder};
use sort_command::exec_sort_command;

use crate::{cli::sort_command::{DRY_RUN, PARAMETER, ROOT_ONLY, STACK}, utils::logger::Logger};

mod cli_handler;
pub mod sort_command;
pub mod error;

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
    .argument(
        STACK.to_string(), 
        String::from("rule that specify where to put the files. first specified element will be the first directory that you will met and so."), 
        vec![ArgValueTypes::Single, ArgValueTypes::Multiple]
    )
    .linked_arg(
        PARAMETER.to_string(), 
        String::from("associate a parameter the the parent stack using a 'KEY=VALUE' format."), 
        vec![ArgValueTypes::Single, ArgValueTypes::Multiple]
    )
    .argument(
        DRY_RUN.to_string(), 
    "Output a report without actually applying it to the files.".to_string(), 
    vec![ArgValueTypes::NoValue]
    )
    .argument(
        ROOT_ONLY.to_string(), 
    "Only compute the report and move file at the input directory root level. Other files will stay in-place.".to_string(), 
    vec![ArgValueTypes::NoValue]
    )
    .parameter(
        String::from("from"), String::from("the directory from which you need to extract the files.")
    )
    .parameter(
        String::from("to"), 
        String::from("the directory to which you need to put the organized files.")
    )
    .handler(|parsed_command, handler_logger| exec_sort_command(parsed_command.args.clone(), parsed_command.params.clone(), handler_logger.clone()))
    .build();

    command_handler.handle(input);
}
