use crate::{
    sortering_strategies::sorting_strategy::SortingStrategy, utils::file_manipulator::move_file,
};
use std::{
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

pub fn sorter(
    input_dir: &str,  // Need to be absolute
    output_dir: &str, // Need to be absolute
    sorting_strategy: SortingStrategy,
    rename_error_handler: fn(&str, &str) -> (),
) {
    let files_list = read_dir(input_dir).unwrap();
    files_list.for_each(|f| {
        let file_name = f.unwrap().file_name();
        let full_filename = Path::new(input_dir).join(file_name.clone());
        let file = File::open(full_filename.clone());
        let file_new_directories_iter = sorting_strategy.iter(file.unwrap());

        let mut new_output = PathBuf::new();
        new_output.push(output_dir);
        for new_directory in file_new_directories_iter {
            new_output.push(new_directory);
        }

        let new_full_filename = new_output.join(file_name);
        match move_file(full_filename.clone(), new_full_filename.clone(), true) {
            Ok(_) => (),
            Err(_) => rename_error_handler(
                full_filename.clone().to_str().unwrap(),
                new_full_filename.clone().to_str().unwrap(),
            ),
        };
    });
}
