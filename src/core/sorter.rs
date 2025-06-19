use std::{
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

use crate::utils::file_manipulator::move_file;

use super::sorting_strategy::SortingStrategy;

pub struct SorterReport {
    pub input_dir: PathBuf,
    pub result: Result<PathBuf, super::error::Error>,
}

/// Need to be absolute
pub fn sorter<'a>(
    input_dir: &str,
    output_dir: &str,
    sorting_strategies: Vec<SortingStrategy>,
) -> Result<Vec<SorterReport>, super::error::Error> {
    let files_list = read_dir(input_dir).map_err(super::error::Error::IO)?;
    let mut reports: Vec<SorterReport> = vec![];

    // validate the strategies
    for strategy in &sorting_strategies {
        strategy
            .validate()
            .map_err(|err| super::error::Error::Validation(strategy.name.clone(), err))?;
    }

    for f in files_list {
        let file_name = f.map_err(super::error::Error::IO).map(|f| f.file_name())?;
        let full_filename = Path::new(input_dir).join(file_name.clone());

        let result = handle_file(
            output_dir,
            &sorting_strategies,
            file_name,
            full_filename.clone(),
        );
        reports.push(SorterReport {
            input_dir: full_filename,
            result,
        });
    }

    Ok(reports)
}

fn handle_file(
    output_dir: &str,
    sorting_strategies: &Vec<SortingStrategy>,
    file_name: std::ffi::OsString,
    full_filename: PathBuf,
) -> Result<PathBuf, super::error::Error> {
    let file = File::open(full_filename.clone()).map_err(super::error::Error::IO)?;

    let mut new_output = PathBuf::new();
    new_output.push(output_dir);
    for strategy in sorting_strategies {
        new_output.push(strategy.apply(&file));
    }

    let new_full_filename = new_output.join(file_name);
    move_file(full_filename.clone(), new_full_filename.clone(), true)
        .map_err(super::error::Error::IO)
        .map(|_| new_full_filename)
}
