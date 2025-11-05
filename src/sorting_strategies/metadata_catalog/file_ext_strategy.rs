use crate::core::{context, strategy};

#[derive(Clone, Debug)]
pub struct FileExtStrategy;

impl FileExtStrategy {
    pub fn new() -> Self {
        FileExtStrategy
    }
}

impl strategy::Apply for FileExtStrategy {
    fn apply(&self, file_path: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
        Some(super::file::file_ext::file_ext(file_path))
    }
}

impl strategy::Validate for FileExtStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        Ok(())
    }
}

impl strategy::Name for FileExtStrategy {
    fn name(&self) -> String {
        "file extension".to_string()
    }
}

impl strategy::AddParameter for FileExtStrategy {
    fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
}

impl strategy::ParameterDetails for FileExtStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        vec![]
    }
}

impl context::ProcessContext for FileExtStrategy {
    fn process_context(
        &mut self,
        _: context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}

impl strategy::Parameters for FileExtStrategy {
    fn parameters(
        &self,
    ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter> {
        std::collections::HashMap::new()
    }
}
