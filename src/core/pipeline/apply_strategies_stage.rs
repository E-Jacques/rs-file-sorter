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

#[cfg(test)]
mod tests {
    use crate::core::{
        pipeline::pipeline_data::PipelineDataKind, sorting_strategy::SortingStrategy,
    };

    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_apply_strategies_should_reject_non_paths_pipeline_data() {
        let stage = ApplyStrategiesStage::new("output".to_string(), vec![]);
        let data = PipelineData::Report(vec![]);

        let result = stage.execute(data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), error::ErrorKind::Pipeline);
    }

    #[test]
    fn test_apply_strategies_should_correctly_handle_single_file_and_no_strategies() {
        let tmp_dir = TempDir::new(
            "test_apply_strategies_should_correctly_handle_single_file_and_no_strategies",
        )
        .expect("Failed to create temp dir");
        let input_file = tmp_dir.path().join("input.txt");
        std::fs::File::create(&input_file).expect("Failed to create input file");
        let stage = ApplyStrategiesStage::new("output".to_string(), vec![]);
        let data = PipelineData::Paths(vec![input_file]);

        let result = stage.execute(data);
        assert!(result.is_ok());
        let unwrapped_data = result.unwrap();
        assert_eq!(unwrapped_data.kind(), PipelineDataKind::Report);
        if let PipelineData::Report(reports) = unwrapped_data {
            assert_eq!(reports.len(), 1);
            for report in reports {
                assert!(report.result.is_ok());
            }
        }
    }

    #[test]
    fn test_apply_strategies_should_correctly_handle_single_file_and_strategies() {
        let tmp_dir = TempDir::new(
            "test_apply_strategies_should_correctly_handle_single_file_and_strategies",
        )
        .expect("Failed to create temp dir");
        let strategy = SortingStrategy::new("strategy1", |_, _, _| "strategy1_output".to_string());
        let strategies = vec![strategy];
        let stage = ApplyStrategiesStage::new(
            tmp_dir
                .path()
                .join("output")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
            strategies,
        );
        let input_path = tmp_dir.path().join("input.txt");
        std::fs::File::create(&input_path).expect("Failed to create input file");
        let data = PipelineData::Paths(vec![input_path]);

        let result = stage.execute(data);
        assert!(result.is_ok());

        let unwrapped_data = result.unwrap();
        assert_eq!(unwrapped_data.kind(), PipelineDataKind::Report);
        if let PipelineData::Report(reports) = unwrapped_data {
            assert_eq!(reports.len(), 1);
            assert!(reports[0].result.is_ok());
            assert_eq!(
                *reports[0].result.as_ref().ok().unwrap(),
                tmp_dir
                    .path()
                    .join("output")
                    .join("strategy1_output")
                    .join("input.txt")
            );
            assert_eq!(reports[0].input_filename, tmp_dir.path().join("input.txt"));
        }

        drop(tmp_dir);
    }

    #[test]
    fn test_apply_strategies_should_correctly_handle_multiple_files_and_no_strategies() {
        let tmp_dir = TempDir::new(
            "test_apply_strategies_should_correctly_handle_multiple_files_and_no_strategies",
        )
        .expect("Failed to create temp dir");
        let input_files = vec![
            tmp_dir.path().join("input1.txt"),
            tmp_dir.path().join("input2.txt"),
        ];
        for file in &input_files {
            std::fs::File::create(file).expect("Failed to create input file");
        }
        let stage = ApplyStrategiesStage::new(
            tmp_dir
                .path()
                .join("output")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
            vec![],
        );
        let data = PipelineData::Paths(input_files);

        let result = stage.execute(data);
        assert!(result.is_ok());
        let unwrapped_data = result.unwrap();
        assert_eq!(unwrapped_data.kind(), PipelineDataKind::Report);
        if let PipelineData::Report(reports) = unwrapped_data {
            assert_eq!(reports.len(), 2);
            assert_eq!(reports[0].input_filename, tmp_dir.path().join("input1.txt"));
            assert_eq!(reports[1].input_filename, tmp_dir.path().join("input2.txt"));
            for report in reports {
                assert!(report.result.is_ok());
                assert_eq!(
                    *report.result.as_ref().ok().unwrap(),
                    tmp_dir
                        .path()
                        .join("output")
                        .join(report.input_filename.file_name().unwrap())
                );
            }
        }

        drop(tmp_dir);
    }

    #[test]
    fn test_apply_strategies_should_correctly_handle_multiple_files_and_strategies() {
        let tmp_dir = TempDir::new(
            "test_apply_strategies_should_correctly_handle_multiple_files_and_strategies",
        )
        .expect("Failed to create temp dir");
        let input_files = vec![
            tmp_dir.path().join("input1.txt"),
            tmp_dir.path().join("input2.txt"),
        ];
        for file in &input_files {
            std::fs::File::create(file).expect("Failed to create input file");
        }
        let tmp_output = tmp_dir.path().join("output");
        let strategy = SortingStrategy::new("strategy1", |_, _, _| "strategy1_output".to_string());
        let strategies = vec![strategy];
        let stage = ApplyStrategiesStage::new(
            tmp_output.as_os_str().to_str().unwrap().to_string(),
            strategies,
        );
        let data = PipelineData::Paths(input_files);

        let result = stage.execute(data);
        assert!(result.is_ok());
        let unwrapped_data = result.unwrap();
        assert_eq!(unwrapped_data.kind(), PipelineDataKind::Report);
        if let PipelineData::Report(reports) = unwrapped_data {
            assert_eq!(reports.len(), 2);
            assert_eq!(reports[0].input_filename, tmp_dir.path().join("input1.txt"));
            assert_eq!(reports[1].input_filename, tmp_dir.path().join("input2.txt"));
            for report in reports {
                assert!(report.result.is_ok());
                assert_eq!(
                    *report.result.as_ref().ok().unwrap(),
                    tmp_output
                        .join("strategy1_output")
                        .join(report.input_filename.file_name().unwrap())
                );
            }
        }

        drop(tmp_dir);
    }
}
