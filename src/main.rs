use rs_file_sorter::cli_handler::cli_handler_builder::CliHandlerBuilder;
use rs_file_sorter::utils::logger::Logger;
use std::env;

fn main() {
    let input = env::args().collect::<Vec<String>>().join(" ");
    let logger = Logger::new("Command Handler", true);
    let command_handler = CliHandlerBuilder::new(logger).build();

    command_handler.handle(input);
}
