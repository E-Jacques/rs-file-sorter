use std::{
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

use crate::utils::{file_manipulator::move_file, logger::Logger};

use super::sorting_strategy::SortingStrategy;

/// Need to be absolute
pub fn sorter<'a>(
    input_dir: &str,
    output_dir: &str,
    sorting_strategies: Vec<SortingStrategy>,
    logger: Logger,
    mut rename_error_handler: impl FnMut(&str, &str) -> (),
) {
    logger.debug(&format!(
        "sorter have been called with input_dir={input_dir} & output_dir={output_dir}."
    ));

    let files_list = read_dir(input_dir).unwrap();
    files_list.for_each(|f| {
        let file_name = f.unwrap().file_name();
        let full_filename = Path::new(input_dir).join(file_name.clone());
        let file = File::open(full_filename.clone()).unwrap();

        let mut new_output = PathBuf::new();
        new_output.push(output_dir);
        for strategy in &sorting_strategies {
            new_output.push(strategy.apply(&file));
        }

        let new_full_filename = new_output.join(file_name);
        match move_file(full_filename.clone(), new_full_filename.clone(), true) {
            Err(_) => rename_error_handler(
                full_filename.clone().to_str().unwrap(),
                new_full_filename.clone().to_str().unwrap(),
            ),
            _ => (),
        };
    });
}
