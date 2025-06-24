use crate::core::{
    error::strategy_validator_error::Error,
    strategy_parameter::{StrategyParameter, StrategyParameterKind},
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StrategyValidator {
    pub kind: StrategyParameterKind,
    pub name: String,
    pub mandatory: bool,
    pub default_value: Option<StrategyParameter>,
}

impl PartialEq for StrategyValidator {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.name == other.name && self.mandatory == other.mandatory
    }
}

impl StrategyValidator {
    pub fn new(name: &str, kind: StrategyParameterKind, mandatory: bool) -> StrategyValidator {
        StrategyValidator {
            name: String::from(name),
            kind,
            mandatory,
            default_value: None,
        }
    }

    pub fn validate(&self, parameters: &HashMap<String, StrategyParameter>) -> Result<(), Error> {
        match parameters.get(&self.name) {
            None if self.mandatory => Err(Error::MissingMandatoryParameter(self.clone())),
            Some(param) if !self.kind.is_matching(param) => Err(Error::TypeError(self.clone())),
            _ => Ok(()),
        }
    }

    pub fn with_default_value(&mut self, default_value: StrategyParameter) -> &mut Self {
        if self.kind.is_matching(&default_value) {
            self.default_value = Some(default_value);
        } else {
            panic!("Default value kind should match with specification.");
        }

        self
    }
}

pub fn parameter_exists(
    parameter_name: &String,
    validators: &Vec<StrategyValidator>,
) -> Result<(), Error> {
    validators
        .iter()
        .any(|v| v.name == *parameter_name)
        .then_some(())
        .ok_or(Error::UnknownParameter(parameter_name.clone()))
}

#[cfg(test)]
mod tests {
    use crate::core::{
        strategy_parameter::StrategyParameterKind, strategy_validator::StrategyValidator,
    };

    #[test]
    fn should_create_strategy_validator() {
        let validator =
            StrategyValidator::new("my-validator", StrategyParameterKind::Strategy, true);

        assert_eq!(validator.name, "my-validator".to_string());
        assert_eq!(validator.kind, StrategyParameterKind::Strategy);
        assert_eq!(validator.mandatory, true);
        assert_eq!(validator.default_value, None);
    }

    mod test_with_default_value {
        use crate::core::{
            strategy_parameter::{StrategyParameter, StrategyParameterKind},
            strategy_validator::StrategyValidator,
        };

        #[test]
        #[should_panic(expected = "Default value kind should match with specification.")]
        fn should_panic_if_default_value_is_conform_to_specification() {
            let _ = StrategyValidator::new("my-validator", StrategyParameterKind::Strategy, true)
                .with_default_value(StrategyParameter::SingleString("my value".to_string()));
        }

        #[test]
        fn should_add_default_value_if_valid() {
            let kind = StrategyParameterKind::Choice(vec![
                "my value".to_string(),
                "another value".to_string(),
            ]);
            let mut validator = StrategyValidator::new("my-validator", kind, true);
            validator.with_default_value(StrategyParameter::SingleString("my value".to_string()));

            assert_eq!(
                validator.default_value,
                Some(StrategyParameter::SingleString("my value".to_string()))
            )
        }
    }

    mod test_strategy_validator_validate {
        use std::collections::HashMap;

        use crate::core::{
            strategy_parameter::{StrategyParameter, StrategyParameterKind},
            strategy_validator::{Error, StrategyValidator},
        };

        #[test]
        fn should_return_ok_if_not_mandatory_and_missing() {
            let validator = StrategyValidator::new(
                "valid-parameter",
                StrategyParameterKind::SingleString,
                false,
            );

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("parameter-1"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(validator.validate(&parameters), Ok(()));
        }

        #[test]
        fn should_return_ok_if_present_and_kind_are_equals() {
            let validator = StrategyValidator::new(
                "valid-parameter",
                StrategyParameterKind::SingleString,
                false,
            );

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("valid-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(validator.validate(&parameters), Ok(()));
        }

        #[test]
        fn should_return_type_error_if_present_but_kind_are_different() {
            let validator =
                StrategyValidator::new("valid-parameter", StrategyParameterKind::Strategy, false);

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("valid-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(
                validator.validate(&parameters),
                Err(Error::TypeError(StrategyValidator::new(
                    "valid-parameter",
                    StrategyParameterKind::Strategy,
                    false,
                )))
            );
        }

        #[test]
        fn should_return_mandatory_error_if_mandatory_but_missing() {
            let validator = StrategyValidator::new(
                "valid-parameter",
                StrategyParameterKind::SingleString,
                true,
            );

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("unknown-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(
                validator.validate(&parameters),
                Err(Error::MissingMandatoryParameter(StrategyValidator::new(
                    "valid-parameter",
                    StrategyParameterKind::SingleString,
                    true,
                )))
            );
        }
    }

    mod test_parameter_exists {
        use crate::core::{
            strategy_parameter::StrategyParameterKind,
            strategy_validator::{parameter_exists, Error, StrategyValidator},
        };

        #[test]
        fn should_return_ok_if_parameter_in_validators() {
            let validators = vec![
                StrategyValidator::new("param-1", StrategyParameterKind::SingleString, false),
                StrategyValidator::new("param-2", StrategyParameterKind::SingleString, true),
            ];

            assert_eq!(
                parameter_exists(&String::from("param-1"), &validators),
                Ok(())
            );
        }

        #[test]
        fn should_return_err_if_validators_empty() {
            assert_eq!(
                parameter_exists(&String::from("param-1"), &vec![]),
                Err(Error::UnknownParameter(String::from("param-1")))
            );
        }

        #[test]
        fn should_return_err_if_parameter_not_in_validators() {
            let validators = vec![
                StrategyValidator::new("param-1", StrategyParameterKind::SingleString, false),
                StrategyValidator::new("param-2", StrategyParameterKind::SingleString, true),
            ];

            assert_eq!(
                parameter_exists(&String::from("param-3"), &validators),
                Err(Error::UnknownParameter(String::from("param-3")))
            );
        }
    }
}
