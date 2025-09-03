use crate::{
    core::{
        parameter::{StrategyParameter, StrategyParameterKind},
        strategy, validation,
    },
    sorting_strategies::utils,
};

#[derive(Clone, Debug)]
pub struct ConcatStrategy {
    validator: utils::BaseValidator,
    parameters: std::collections::HashMap<String, StrategyParameter>,
}

impl ConcatStrategy {
    pub fn new() -> Self {
        let mut validator = utils::BaseValidator::new();
        validator.add_validator(validation::ParameterDetail::new(
            "strategies",
            StrategyParameterKind::Strategy,
            true,
        ));

        let parameters = validator.default_parameters();

        ConcatStrategy {
            validator,
            parameters,
        }
    }
}

impl strategy::AddParameter for ConcatStrategy {
    fn add_parameter(&mut self, key: String, value: StrategyParameter) {
        self.parameters.insert(key, value);
    }
}

impl strategy::Validate for ConcatStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        self.validator.validate(&self.parameters)
    }
}

impl strategy::ParameterDetails for ConcatStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        self.validator.parameter_details()
    }
}

impl strategy::Name for ConcatStrategy {
    fn name(&self) -> String {
        "concat".to_string()
    }
}

impl strategy::Apply for ConcatStrategy {
    fn apply(&self, file_path: &std::path::PathBuf, f: &std::fs::File) -> Option<String> {
        let mut result = String::new();

        let strategies = self.parameters.get("strategies").unwrap();
        match strategies {
            StrategyParameter::Strategy(strategies) => {
                strategies
                    .iter()
                    .filter_map(|strategy| strategy.apply(file_path, f))
                    .for_each(|part| result.push_str(&part));
            }
            _ => (),
        }
        Some(result)
    }
}

impl crate::core::context::ProcessContext for ConcatStrategy {
    fn process_context(
        &mut self,
        _: crate::core::context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}
