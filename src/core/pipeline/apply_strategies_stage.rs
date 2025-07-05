use std::{fs, path::PathBuf};

use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct ApplyStrategiesStage {
    output: String,
    strategies: Vec<crate::core::sorting_strategy::SortingStrategy>,
}

impl ApplyStrategiesStage {
    pub fn new(
        output: String,
        strategies: Vec<crate::core::sorting_strategy::SortingStrategy>,
    ) -> Self {
        ApplyStrategiesStage { output, strategies }
    }

    fn apply_strategies(
        &self,
        file_name: &std::ffi::OsStr,
        full_filename: &PathBuf,
    ) -> Result<PathBuf, error::Error> {
        let file = fs::File::open(full_filename.clone()).map_err(error::Error::IO)?;

        let mut new_output = PathBuf::new();
        new_output.push(&self.output);
        for strategy in &self.strategies {
            new_output.push(strategy.apply(full_filename, &file));
        }

        Ok(new_output.join(file_name))
    }
}

impl PipelineStage<PipelineData, error::Error> for ApplyStrategiesStage {
    fn execute(&self, data: PipelineData) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Paths(path_bufs) => {
                let mut reports: crate::core::sorter::FullSorterReport = vec![];
                for file in path_bufs {
                    let file_name = file.file_name().unwrap_or(&std::ffi::OsStr::new("/"));

                    let result = self.apply_strategies(file_name, &file);
                    reports.push(crate::core::sorter::SorterReport {
                        input_filename: file.clone(),
                        result: result.map_err(std::rc::Rc::new),
                    });
                }

                Ok(PipelineData::Report(reports))
            }
            _ => Err(error::Error::Pipeline),
        }
    }
}

impl std::fmt::Display for ApplyStrategiesStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Computing new file names...")
    }
}
