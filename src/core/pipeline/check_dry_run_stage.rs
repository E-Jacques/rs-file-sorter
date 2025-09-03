use crate::core::pipeline::pipeline_data::PipelineContext;

use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct CheckDryRunStage;
impl PipelineStage<PipelineData, error::Error> for CheckDryRunStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        if context.options().dry_run {
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
            pipeline_data::{PipelineContext, PipelineData, PipelineDataKind},
            stage::PipelineStage,
        },
        options::SortOptions,
    };

    #[test]
    fn test_should_return_pause_if_dry_run_is_true() {
        let mut context: PipelineContext = PipelineContext::default();
        context.set_options(SortOptions {
            dry_run: true,
            root_level_only: false,
        });

        let result = CheckDryRunStage.execute(context, PipelineData::Report(vec![]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), PipelineDataKind::Pause);
    }

    #[test]
    fn test_should_return_data_if_dry_run_is_false() {
        let mut context = PipelineContext::default();
        context.set_options(SortOptions {
            dry_run: false,
            root_level_only: false,
        });

        let result = CheckDryRunStage.execute(context, PipelineData::Report(vec![]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), PipelineDataKind::Report);
    }
}
