use crate::core::strategy;

#[derive(Clone, Debug)]
pub struct FileTypeStrategy;

impl FileTypeStrategy {
    pub fn new() -> Self {
        FileTypeStrategy
    }
}

impl strategy::Apply for FileTypeStrategy {
    fn apply(&self, file_path: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
        let ext = super::file::file_ext::file_ext(file_path);
        Some(super::file::filetype::FileType::from_extension(&ext).to_string())
    }
}

impl strategy::Validate for FileTypeStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        Ok(())
    }
}

impl strategy::Name for FileTypeStrategy {
    fn name(&self) -> String {
        "file type".to_string()
    }
}

impl strategy::AddParameter for FileTypeStrategy {
    fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
}

impl strategy::ParameterDetails for FileTypeStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        vec![]
    }
}

impl crate::core::context::ProcessContext for FileTypeStrategy {
    fn process_context(
        &mut self,
        _: crate::core::context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}
