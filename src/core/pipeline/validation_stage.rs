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

#[cfg(test)]
mod tests {
    use crate::{
        core::{
            pipeline::pipeline_data::PipelineDataKind, sorting_strategy::SortingStrategy,
            strategy_parameter::StrategyParameterKind, strategy_validator::StrategyValidator,
        },
        sorting_strategies::metadata_catalog::get_metadata_catalog,
    };

    use super::*;

    #[test]
    fn test_should_return_empty_if_valid() {
        let catalog = get_metadata_catalog();
        let strategies = catalog
            .get_names()
            .iter()
            .filter_map(|name| catalog.get_strategy(name))
            .collect();
        let pipeline_stage = ValidationStage::new(strategies);

        let data = pipeline_stage.execute(PipelineData::Empty);
        assert!(data.is_ok());
        assert_eq!(data.unwrap().kind(), PipelineDataKind::Empty)
    }

    #[test]
    fn test_should_return_an_error_if_input_non_empty_data() {
        let catalog = get_metadata_catalog();
        let strategies = catalog
            .get_names()
            .iter()
            .filter_map(|name| catalog.get_strategy(name))
            .collect();
        let pipeline_stage = ValidationStage::new(strategies);

        let data = pipeline_stage.execute(PipelineData::Paths(vec![]));
        assert!(data.is_err());
        assert_eq!(data.unwrap_err().kind(), super::error::ErrorKind::Pipeline)
    }

    #[test]
    fn test_should_return_validation_error_if_not_valid() {
        let mut strategy = SortingStrategy::new("my-strategy", |_, _, _| String::from("test"));
        strategy.add_validator(StrategyValidator::new(
            "my-validator",
            StrategyParameterKind::SingleString,
            true,
        ));

        let pipeline_stage = ValidationStage::new(vec![strategy]);
        let data = pipeline_stage.execute(PipelineData::Empty);
        assert!(data.is_err());
        assert_eq!(
            data.unwrap_err().kind(),
            super::error::ErrorKind::Validation
        );
    }
}
