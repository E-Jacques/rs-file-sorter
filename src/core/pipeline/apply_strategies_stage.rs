use std::{fs, path::PathBuf};

use super::super::error;
use crate::core::pipeline::pipeline_data::PipelineContext;

use super::{stage::PipelineStage, PipelineData};

pub struct ApplyStrategiesStage;
impl ApplyStrategiesStage {
    fn apply_strategies(
        &self,
        context: &PipelineContext,
        file_name: &std::ffi::OsStr,
        full_filename: &PathBuf,
    ) -> Result<PathBuf, error::Error> {
        let file = fs::File::open(full_filename.clone()).map_err(error::Error::IO)?;

        let mut new_output = PathBuf::from(context.output_dir());
        context
            .strategies()
            .iter()
            .filter_map(|strategy| strategy.apply(full_filename, &file))
            .for_each(|path| new_output.push(path));

        Ok(new_output.join(file_name))
    }
}

impl PipelineStage<PipelineData, error::Error> for ApplyStrategiesStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Paths(path_bufs) => {
                let mut reports: crate::core::report::FullReport = vec![];
                for file in path_bufs {
                    let file_name = file.file_name().unwrap_or(&std::ffi::OsStr::new("/"));

                    let result = self.apply_strategies(&context, file_name, &file);
                    reports.push(crate::core::report::Report {
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
        context::ProcessContext,
        options::SortOptions,
        pipeline::pipeline_data::PipelineDataKind,
        strategy::{AddParameter, Apply, Name, ParameterDetails, Parameters, Validate},
        validation,
    };

    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_apply_strategies_should_reject_non_paths_pipeline_data() {
        let context = PipelineContext::new(
            vec![],
            SortOptions::default(),
            "input".to_string(),
            "output".to_string(),
        );
        let data = PipelineData::Report(vec![]);

        let result = ApplyStrategiesStage.execute(context, data);
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
        let context = PipelineContext::new(
            vec![],
            SortOptions::default(),
            "input".to_string(),
            "output".to_string(),
        );
        let data = PipelineData::Paths(vec![input_file]);

        let result = ApplyStrategiesStage.execute(context, data);
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

        #[derive(Clone, Debug)]
        struct Strategy1;
        impl Apply for Strategy1 {
            fn apply(&self, _file: &PathBuf, _input: &std::fs::File) -> Option<String> {
                Some("strategy1_output".to_string())
            }
        }

        impl Validate for Strategy1 {
            fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
                Ok(())
            }
        }

        impl ProcessContext for Strategy1 {
            fn process_context(
                &mut self,
                _: crate::core::context::StrategyContext,
            ) -> Result<(), error::Error> {
                Ok(())
            }
        }

        impl AddParameter for Strategy1 {
            fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
        }

        impl ParameterDetails for Strategy1 {
            fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
                vec![]
            }
        }

        impl Name for Strategy1 {
            fn name(&self) -> String {
                "strategy1".to_string()
            }
        }

        impl Parameters for Strategy1 {
            fn parameters(
                &self,
            ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter>
            {
                std::collections::HashMap::new()
            }
        }

        let context = PipelineContext::new(
            vec![Box::new(Strategy1)],
            SortOptions::default(),
            "input".to_string(),
            tmp_dir
                .path()
                .join("output")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
        );
        let input_path = tmp_dir.path().join("input.txt");
        std::fs::File::create(&input_path).expect("Failed to create input file");
        let data = PipelineData::Paths(vec![input_path]);

        let result = ApplyStrategiesStage.execute(context, data);
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
        let context = PipelineContext::new(
            vec![],
            SortOptions::default(),
            "input".to_string(),
            tmp_dir
                .path()
                .join("output")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
        );
        let data = PipelineData::Paths(input_files);

        let result = ApplyStrategiesStage.execute(context, data);
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

        #[derive(Clone, Debug)]
        struct Strategy1;
        impl Apply for Strategy1 {
            fn apply(&self, _file: &PathBuf, _input: &std::fs::File) -> Option<String> {
                Some("strategy1_output".to_string())
            }
        }

        impl ProcessContext for Strategy1 {
            fn process_context(
                &mut self,
                _: crate::core::context::StrategyContext,
            ) -> Result<(), error::Error> {
                Ok(())
            }
        }

        impl Validate for Strategy1 {
            fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
                Ok(())
            }
        }

        impl AddParameter for Strategy1 {
            fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
        }

        impl ParameterDetails for Strategy1 {
            fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
                vec![]
            }
        }

        impl Name for Strategy1 {
            fn name(&self) -> String {
                "strategy1".to_string()
            }
        }

        impl Parameters for Strategy1 {
            fn parameters(
                &self,
            ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter>
            {
                std::collections::HashMap::new()
            }
        }

        let context = PipelineContext::new(
            vec![Box::new(Strategy1)],
            SortOptions::default(),
            "input".to_string(),
            tmp_output.as_os_str().to_str().unwrap().to_string(),
        );
        let data = PipelineData::Paths(input_files);

        let result = ApplyStrategiesStage.execute(context, data);
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

    #[test]
    fn test_apply_strategies_should_ignore_none_return() {
        let tmp_dir = TempDir::new("test_apply_strategies_should_ignore_none_return")
            .expect("Failed to create temp dir");
        let input_file = tmp_dir.path().join("input.txt");
        std::fs::File::create(&input_file).expect("Failed to create input file");

        #[derive(Clone, Debug)]
        struct Strategy1;
        impl Apply for Strategy1 {
            fn apply(&self, _file: &PathBuf, _input: &std::fs::File) -> Option<String> {
                None
            }
        }

        impl Validate for Strategy1 {
            fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
                Ok(())
            }
        }

        impl ProcessContext for Strategy1 {
            fn process_context(
                &mut self,
                _: crate::core::context::StrategyContext,
            ) -> Result<(), error::Error> {
                Ok(())
            }
        }

        impl AddParameter for Strategy1 {
            fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
        }

        impl ParameterDetails for Strategy1 {
            fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
                vec![]
            }
        }

        impl Name for Strategy1 {
            fn name(&self) -> String {
                "strategy1".to_string()
            }
        }

        impl Parameters for Strategy1 {
            fn parameters(
                &self,
            ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter>
            {
                std::collections::HashMap::new()
            }
        }

        let context = PipelineContext::new(
            vec![Box::new(Strategy1)],
            SortOptions::default(),
            "input".to_string(),
            tmp_dir
                .path()
                .join("output")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
        );
        let data = PipelineData::Paths(vec![input_file]);

        let result = ApplyStrategiesStage.execute(context, data);
        assert!(result.is_ok());
        let unwrapped_data = result.unwrap();
        assert_eq!(unwrapped_data.kind(), PipelineDataKind::Report);
        if let PipelineData::Report(reports) = unwrapped_data {
            assert_eq!(reports.len(), 1);
            assert!(reports[0].result.is_ok());
            assert_eq!(
                *reports[0].result.as_ref().ok().unwrap(),
                tmp_dir.path().join("output").join("input.txt")
            );
            assert_eq!(reports[0].input_filename, tmp_dir.path().join("input.txt"));
        }

        drop(tmp_dir);
    }
}
