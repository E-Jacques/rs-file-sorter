use crate::{
    core::{
        parameter::{StrategyParameter, StrategyParameterKind},
        strategy, validation,
    },
    sorting_strategies::utils,
};

#[derive(Clone, Debug)]
pub struct TextStrategy {
    validator: utils::BaseValidator,
    parameters: std::collections::HashMap<String, StrategyParameter>,
}

impl TextStrategy {
    pub fn new() -> Self {
        let mut validator = utils::BaseValidator::new();
        validator.add_validator(validation::ParameterDetail::new(
            "value",
            StrategyParameterKind::SingleString,
            true,
        ));

        let parameters = validator.default_parameters();

        TextStrategy {
            validator,
            parameters,
        }
    }
}

impl strategy::AddParameter for TextStrategy {
    fn add_parameter(&mut self, key: String, value: StrategyParameter) {
        self.parameters.insert(key, value);
    }
}

impl strategy::Validate for TextStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        self.validator.validate(&self.parameters)
    }
}

impl strategy::ParameterDetails for TextStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        self.validator.parameter_details()
    }
}

impl strategy::Name for TextStrategy {
    fn name(&self) -> String {
        "text".to_string()
    }
}

impl strategy::Apply for TextStrategy {
    fn apply(&self, _: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
        let mut result = String::new();

        let strategies = self.parameters.get("value").unwrap();
        match strategies {
            StrategyParameter::SingleString(value) => {
                result = value.clone();
            }
            _ => (),
        }
        Some(result)
    }
}

impl crate::core::context::ProcessContext for TextStrategy {
    fn process_context(
        &mut self,
        _: crate::core::context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}

impl strategy::AsStrategy for TextStrategy {
    fn as_strategy(&self) -> &dyn strategy::Strategy {
        self
    }
}
