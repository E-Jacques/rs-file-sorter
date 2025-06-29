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
}

impl SortPipeline {
    pub fn new(
        input: String,
        output: String,
        strategies: Vec<super::sorting_strategy::SortingStrategy>,
        options: super::sorter::SortOptions,
    ) -> Self {
        let stages: Vec<Box<dyn PipelineStageDisplay>> = vec![
            Box::new(ValidationStage::new(strategies.clone())),
            Box::new(GetFilesStage::new(options.clone(), input.clone())),
            Box::new(ApplyStrategiesStage::new(
                output.clone(),
                strategies.clone(),
            )),
            Box::new(CheckDryRunStage::new(options.clone())),
            Box::new(ApplyTransformationStage::new(
                options.clone(),
                input.clone(),
            )),
        ];

        SortPipeline {
            curr: 0,
            stages,
            data: PipelineData::Empty,
        }
    }

    pub fn process(
        &mut self,
    ) -> Result<Option<super::sorter::FullSorterReport>, super::error::Error> {
        let mut report: Option<super::sorter::FullSorterReport> = None;

        while self.has_next() {
            if let Some(boxed_stage) = self.stages.get(self.curr) {
                let stage: &dyn PipelineStageDisplay = &**boxed_stage;

                self.curr = self.curr + 1;
                match stage.execute(self.data.clone())? {
                    PipelineData::Pause => break,
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
