use crate::{
    core::{
        context,
        parameter::{StrategyParameter, StrategyParameterKind},
        strategy, validation,
    },
    sorting_strategies::utils,
};

#[derive(Clone, Debug)]
pub struct OrStrategy {
    validator: utils::BaseValidator,
    parameters: std::collections::HashMap<String, StrategyParameter>,
}

impl OrStrategy {
    pub fn new() -> Self {
        let mut validator = utils::BaseValidator::new();
        validator.add_validator(validation::ParameterDetail::new(
            "strategies",
            StrategyParameterKind::Strategy,
            true,
        ));

        let parameters = validator.default_parameters();

        OrStrategy {
            validator,
            parameters,
        }
    }
}

impl strategy::AsStrategy for OrStrategy {
    fn as_strategy(&self) -> &dyn strategy::Strategy {
        self
    }
}

impl strategy::AddParameter for OrStrategy {
    fn add_parameter(&mut self, key: String, value: StrategyParameter) {
        self.parameters.insert(key, value);
    }
}

impl strategy::Validate for OrStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        self.validator.validate(&self.parameters)
    }
}

impl strategy::ParameterDetails for OrStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        self.validator.parameter_details()
    }
}

impl strategy::Parameters for OrStrategy {
    fn parameters(&self) -> std::collections::HashMap<String, StrategyParameter> {
        self.parameters.clone()
    }
}

impl strategy::Name for OrStrategy {
    fn name(&self) -> String {
        "or".to_string()
    }
}

impl strategy::Apply for OrStrategy {
    fn apply(&self, file_path: &std::path::PathBuf, f: &std::fs::File) -> Option<String> {
        let strategies = self
            .parameters
            .get("strategies")
            .and_then(|param| match param {
                StrategyParameter::Strategy(list) => Some(list),
                _ => None,
            })?;

        for strategy in strategies {
            if let Some(result) = strategy.apply(file_path, f) {
                return Some(result);
            }
        }

        None
    }
}

impl context::ProcessContext for OrStrategy {
    fn process_context(
        &mut self,
        _: context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::strategy::Strategy,
        sorting_strategies::manipulation_catalog::text_strategy::TextStrategy,
    };

    #[derive(Clone, Debug)]
    struct NoneStrategy;

    impl strategy::Apply for NoneStrategy {
        fn apply(&self, _: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
            None
        }
    }

    impl strategy::Name for NoneStrategy {
        fn name(&self) -> String {
            "none".to_string()
        }
    }

    impl strategy::Validate for NoneStrategy {
        fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
            Ok(())
        }
    }

    impl context::ProcessContext for NoneStrategy {
        fn process_context(
            &mut self,
            _: context::StrategyContext,
        ) -> Result<(), crate::core::error::Error> {
            Ok(())
        }
    }

    impl strategy::ParameterDetails for NoneStrategy {
        fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
            vec![]
        }
    }

    impl strategy::AddParameter for NoneStrategy {
        fn add_parameter(&mut self, _: String, _: StrategyParameter) {}
    }

    impl strategy::AsStrategy for NoneStrategy {
        fn as_strategy(&self) -> &dyn strategy::Strategy {
            self
        }
    }

    impl strategy::Parameters for NoneStrategy {
        fn parameters(&self) -> std::collections::HashMap<String, StrategyParameter> {
            std::collections::HashMap::new()
        }
    }

    #[test]
    fn should_return_a_validation_error_if_no_strategies_are_provided() {
        let strategy = OrStrategy::new();
        let result: Result<(), validation::error::Error> = strategy.as_validate().validate();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(
            error,
            validation::error::Error::MissingMandatoryParameter(validation::ParameterDetail::new(
                "strategies",
                StrategyParameterKind::Strategy,
                true
            ))
        );
    }

    #[test]
    fn should_return_first_ok_value() {
        let text_strategy: &mut dyn strategy::Strategy = &mut TextStrategy::new();
        text_strategy.add_parameter(
            "value".to_string(),
            StrategyParameter::SingleString("first".to_string()),
        );

        let or_strategy: &mut dyn strategy::Strategy = &mut OrStrategy::new();
        or_strategy.add_parameter(
            "strategies".to_string(),
            StrategyParameter::Strategy(vec![
                Box::new(NoneStrategy),
                Box::new(NoneStrategy),
                text_strategy.clone_box(),
            ]),
        );

        let result = or_strategy.as_apply().apply(
            &std::path::PathBuf::new(),
            &std::fs::File::open("Cargo.toml").unwrap(),
        );
        assert_eq!(result, Some("first".to_string()));
    }

    #[test]
    fn should_return_none_if_all_strategies_return_none() {
        let or_strategy: &mut dyn strategy::Strategy = &mut OrStrategy::new();
        or_strategy.add_parameter(
            "strategies".to_string(),
            StrategyParameter::Strategy(vec![
                Box::new(NoneStrategy),
                Box::new(NoneStrategy),
                Box::new(NoneStrategy),
            ]),
        );

        let result = or_strategy.as_apply().apply(
            &std::path::PathBuf::new(),
            &std::fs::File::open("Cargo.toml").unwrap(),
        );
        assert_eq!(result, None);
    }
}
