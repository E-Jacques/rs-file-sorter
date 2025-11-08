use crate::core::pipeline::pipeline_data::PipelineContext;

use super::super::error;

use super::{stage::PipelineStage, PipelineData};
pub struct ValidationStage;
impl PipelineStage<PipelineData, error::Error> for ValidationStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Empty => {
                // validate the strategies
                for strategy in &context.strategies() {
                    strategy.validate().map_err(error::Error::Validation)?;
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
            context::ProcessContext,
            pipeline::pipeline_data::PipelineDataKind,
            strategy::{AddParameter, Apply, Name, ParameterDetails, Parameters, Strategy},
        },
        sorting_strategies::catalog::get_metadata_catalog,
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

        let mut context = PipelineContext::default();
        context.set_strategies(strategies);
        let data = ValidationStage.execute(context, PipelineData::Empty);
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

        let mut context = PipelineContext::default();
        context.set_strategies(strategies);

        let data = ValidationStage.execute(context, PipelineData::Paths(vec![]));
        assert!(data.is_err());
        assert_eq!(data.unwrap_err().kind(), super::error::ErrorKind::Pipeline)
    }

    #[test]
    fn test_should_return_validation_error_if_not_valid() {
        #[derive(Debug, Clone)]
        struct MyStrategy;
        impl crate::core::strategy::Validate for MyStrategy {
            fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
                Err(crate::core::validation::error::Error::UnknownParameter(
                    String::from("unknown-param"),
                ))
            }
        }

        impl Apply for MyStrategy {
            fn apply(&self, _: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
                None
            }
        }

        impl ProcessContext for MyStrategy {
            fn process_context(
                &mut self,
                _: crate::core::context::StrategyContext,
            ) -> Result<(), error::Error> {
                Ok(())
            }
        }

        impl AddParameter for MyStrategy {
            fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
        }

        impl ParameterDetails for MyStrategy {
            fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
                vec![]
            }
        }

        impl Name for MyStrategy {
            fn name(&self) -> String {
                "my-strategy".to_string()
            }
        }

        impl Parameters for MyStrategy {
            fn parameters(
                &self,
            ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter>
            {
                std::collections::HashMap::new()
            }
        }

        let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(MyStrategy)];
        let mut context = PipelineContext::default();
        context.set_strategies(strategies);
        let data = ValidationStage.execute(context, PipelineData::Empty);
        assert!(data.is_err());
        assert_eq!(
            data.unwrap_err().kind(),
            super::error::ErrorKind::Validation
        );
    }
}
