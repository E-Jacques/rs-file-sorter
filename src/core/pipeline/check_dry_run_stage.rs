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

#[cfg(test)]
mod tests {
    use crate::core::{
        pipeline::{
            check_dry_run_stage::CheckDryRunStage,
            pipeline_data::{PipelineData, PipelineDataKind},
            stage::PipelineStage,
        },
        sorter::SortOptions,
    };

    #[test]
    fn test_should_return_pause_if_dry_run_is_true() {
        let options: SortOptions = SortOptions {
            dry_run: true,
            root_level_only: false,
        };
        let pipeline_stage = CheckDryRunStage::new(options);

        let result = pipeline_stage.execute(PipelineData::Report(vec![]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), PipelineDataKind::Pause);
    }

    #[test]
    fn test_should_return_data_if_dry_run_is_false() {
        let options: SortOptions = SortOptions {
            dry_run: false,
            root_level_only: false,
        };
        let pipeline_stage = CheckDryRunStage::new(options);

        let result = pipeline_stage.execute(PipelineData::Report(vec![]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), PipelineDataKind::Report);
    }
}
