use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct ValidationStage {
    strategies: Vec<crate::core::sorting_strategy::SortingStrategy>,
}

impl ValidationStage {
    pub fn new(strategies: Vec<crate::core::sorting_strategy::SortingStrategy>) -> Self {
        ValidationStage { strategies }
    }
}

impl PipelineStage<PipelineData, error::Error> for ValidationStage {
    fn execute(&self, data: PipelineData) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Empty => {
                // validate the strategies
                for strategy in &self.strategies {
                    strategy
                        .validate()
                        .map_err(|err| error::Error::Validation(strategy.name.clone(), err))?;
                }

                Ok(PipelineData::Empty)
            }
            _ => Err(error::Error::Pipeline),
        }
    }
}

impl std::fmt::Display for ValidationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validating parameters...")
    }
}
