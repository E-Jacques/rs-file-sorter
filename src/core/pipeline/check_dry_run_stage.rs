use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct CheckDryRunStage {
    options: crate::core::sorter::SortOptions,
}

impl CheckDryRunStage {
    pub fn new(options: crate::core::sorter::SortOptions) -> Self {
        CheckDryRunStage { options }
    }
}

impl PipelineStage<PipelineData, error::Error> for CheckDryRunStage {
    fn execute(&self, data: PipelineData) -> Result<PipelineData, error::Error> {
        if self.options.dry_run {
            Ok(PipelineData::Pause)
        } else {
            Ok(data)
        }
    }
}

impl std::fmt::Display for CheckDryRunStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checking if user validation is required")
    }
}
