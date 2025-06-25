use std::{
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

use crate::utils::file_manipulator::move_file;

use super::sorting_strategy::SortingStrategy;

pub type FullSorterReport = Vec<SorterReport>;

#[derive(Clone, Debug)]
pub struct SorterReport {
    pub input_filename: PathBuf,
    pub result: Result<PathBuf, std::rc::Rc<super::error::Error>>,
}

#[derive(Clone, Debug)]
pub struct SortOptions {
    pub dry_run: bool,
}

impl Default for SortOptions {
    fn default() -> Self {
        SortOptions { dry_run: false }
    }
}

/// Need to be absolute
pub fn sorter<'a>(
    input_dir: &str,
    output_dir: &str,
    sorting_strategies: Vec<SortingStrategy>,
    options: &SortOptions,
) -> Result<FullSorterReport, super::error::Error> {
    let files_list = read_dir(input_dir).map_err(super::error::Error::IO)?;
    let mut reports: FullSorterReport = vec![];

    // validate the strategies
    for strategy in &sorting_strategies {
        strategy
            .validate()
            .map_err(|err| super::error::Error::Validation(strategy.name.clone(), err))?;
    }

    for f in files_list {
        let file_name = f.map_err(super::error::Error::IO).map(|f| f.file_name())?;
        let full_filename = Path::new(input_dir).join(file_name.clone());

        let result = apply_strategies(
            output_dir,
            &sorting_strategies,
            file_name,
            full_filename.clone(),
        );
        reports.push(SorterReport {
            input_filename: full_filename,
            result: result.map_err(std::rc::Rc::new),
        });
    }

    if !options.dry_run {
        reports = move_files_from_report(reports);
    }

    Ok(reports)
}

pub fn move_files_from_report(mut reports: FullSorterReport) -> FullSorterReport {
    for report in reports.iter_mut() {
        println!("{:?}", report);
        if let Ok(target) = &mut report.result {
            report.result = move_file(&report.input_filename, target, true)
                .map_err(super::error::Error::IO)
                .map_err(std::rc::Rc::new)
                .map(|_| target.clone());
        }
    }

    reports
}

fn apply_strategies(
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

    Ok(new_output.join(file_name))
}
