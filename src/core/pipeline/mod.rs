pub mod apply_context_stage;
pub mod apply_strategies_stage;
pub mod apply_transformation_stage;
pub mod check_dry_run_stage;
pub mod get_files_stage;
pub mod pipeline_data;
mod stage;
pub mod validation_stage;

use apply_strategies_stage::ApplyStrategiesStage;
use apply_transformation_stage::ApplyTransformationStage;
use check_dry_run_stage::CheckDryRunStage;
use get_files_stage::GetFilesStage;
use pipeline_data::PipelineData;
use validation_stage::ValidationStage;

use crate::core::{pipeline::pipeline_data::PipelineContext, strategy::Strategy};

trait PipelineStageDisplay:
    stage::PipelineStage<PipelineData, super::error::Error> + std::fmt::Display
{
}

impl<T> PipelineStageDisplay for T where
    T: stage::PipelineStage<PipelineData, super::error::Error> + std::fmt::Display
{
}

pub struct SortPipeline {
    curr: usize,
    stages: Vec<Box<dyn PipelineStageDisplay>>,
    data: PipelineData,
    context: PipelineContext,
}

impl SortPipeline {
    pub fn new(
        input: String,
        output: String,
        strategies: Vec<Box<dyn Strategy>>,
        options: super::options::SortOptions,
    ) -> Self {
        let context = PipelineContext::new(strategies, options, input.clone(), output.clone());
        let stages: Vec<Box<dyn PipelineStageDisplay>> = vec![
            Box::new(ValidationStage),
            Box::new(GetFilesStage),
            Box::new(ApplyContextStage),
            Box::new(ApplyStrategiesStage),
            Box::new(CheckDryRunStage),
            Box::new(ApplyTransformationStage),
        ];

        SortPipeline {
            curr: 0,
            stages,
            data: PipelineData::Empty,
            context,
        }
    }

    pub fn process(&mut self) -> Result<Option<super::report::FullReport>, super::error::Error> {
        let mut report: Option<super::report::FullReport> = None;

        while self.has_next() {
            if let Some(boxed_stage) = self.stages.get(self.curr) {
                let stage: &dyn PipelineStageDisplay = &**boxed_stage;

                self.curr = self.curr + 1;
                match stage.execute(self.context.clone(), self.data.clone())? {
                    PipelineData::Pause => break,
                    PipelineData::Context(context) => self.context = context,
                    data => self.data = data,
                };

                if let PipelineData::Report(inner_data) = &self.data {
                    report = Some(inner_data.clone())
                }
            } else {
                break;
            }
        }

        Ok(report)
    }

    pub fn has_next(&self) -> bool {
        self.curr < self.stages.len()
    }
}
