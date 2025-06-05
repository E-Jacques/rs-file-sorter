use crate::core::strategy_parameter::{StrategyParameter, StrategyParameterKind};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StrategyValidator {
    pub kind: StrategyParameterKind,
    pub name: String,
    pub mandatory: bool,
}

impl PartialEq for StrategyValidator {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.name == other.name && self.mandatory == other.mandatory
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrategyValidatorError {
    MissingMandatoryParameter(StrategyValidator),
    UnknownParameter(String),
    TypeError(StrategyValidator),
}

impl StrategyValidator {
    pub fn new(name: &str, kind: StrategyParameterKind, mandatory: bool) -> StrategyValidator {
        StrategyValidator {
            name: String::from(name),
            kind,
            mandatory,
        }
    }

    pub fn validate(
        &self,
        parameters: &HashMap<String, StrategyParameter>,
    ) -> Result<(), StrategyValidatorError> {
        match parameters.get(&self.name) {
            None if self.mandatory => Err(StrategyValidatorError::MissingMandatoryParameter(
                self.clone(),
            )),
            Some(param) if param.kind() != self.kind => {
                Err(StrategyValidatorError::TypeError(self.clone()))
            }
            _ => Ok(()),
        }
    }
}

pub fn parameter_exists(
    parameter_name: &String,
    validators: &Vec<StrategyValidator>,
) -> Result<(), StrategyValidatorError> {
    validators
        .iter()
        .any(|v| v.name == *parameter_name)
        .then_some(())
        .ok_or(StrategyValidatorError::UnknownParameter(
            parameter_name.clone(),
        ))
}

#[cfg(test)]
mod tests {

    mod test_strategy_validator_validate {
        use std::collections::HashMap;

        use crate::core::{
            strategy_parameter::{StrategyParameter, StrategyParameterKind},
            strategy_validator::{StrategyValidator, StrategyValidatorError},
        };

        #[test]
        fn should_return_ok_if_not_mandatory_and_missing() {
            let validator = StrategyValidator {
                name: String::from("valid-parameter"),
                mandatory: false,
                kind: StrategyParameterKind::SingleString,
            };

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("parameter-1"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(validator.validate(&parameters), Ok(()));
        }

        #[test]
        fn should_return_ok_if_present_and_kind_are_equals() {
            let validator = StrategyValidator {
                name: String::from("valid-parameter"),
                mandatory: false,
                kind: StrategyParameterKind::SingleString,
            };

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("valid-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(validator.validate(&parameters), Ok(()));
        }

        #[test]
        fn should_return_type_error_if_present_but_kind_are_different() {
            let validator = StrategyValidator {
                name: String::from("valid-parameter"),
                mandatory: false,
                kind: StrategyParameterKind::Strategy,
            };

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("valid-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(
                validator.validate(&parameters),
                Err(StrategyValidatorError::TypeError(StrategyValidator {
                    name: String::from("valid-parameter"),
                    mandatory: false,
                    kind: StrategyParameterKind::Strategy,
                }))
            );
        }

        #[test]
        fn should_return_mandatory_error_if_mandatory_but_missing() {
            let validator = StrategyValidator {
                name: String::from("valid-parameter"),
                mandatory: true,
                kind: StrategyParameterKind::SingleString,
            };

            let mut parameters: HashMap<String, StrategyParameter> = HashMap::new();
            parameters.insert(
                String::from("unknown-parameter"),
                StrategyParameter::SingleString(String::from("My_value")),
            );

            assert_eq!(
                validator.validate(&parameters),
                Err(StrategyValidatorError::MissingMandatoryParameter(
                    StrategyValidator {
                        name: String::from("valid-parameter"),
                        mandatory: true,
                        kind: StrategyParameterKind::SingleString,
                    }
                ))
            );
        }
    }

    mod test_parameter_exists {
        use crate::core::{
            strategy_parameter::StrategyParameterKind,
            strategy_validator::{parameter_exists, StrategyValidator, StrategyValidatorError},
        };

        #[test]
        fn should_return_ok_if_parameter_in_validators() {
            let validators = vec![
                StrategyValidator {
                    kind: StrategyParameterKind::SingleString,
                    name: String::from("param-1"),
                    mandatory: false,
                },
                StrategyValidator {
                    kind: StrategyParameterKind::SingleString,
                    name: String::from("param-2"),
                    mandatory: true,
                },
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
                Err(StrategyValidatorError::UnknownParameter(String::from(
                    "param-1"
                )))
            );
        }

        #[test]
        fn should_return_err_if_parameter_not_in_validators() {
            let validators = vec![
                StrategyValidator {
                    kind: StrategyParameterKind::SingleString,
                    name: String::from("param-1"),
                    mandatory: false,
                },
                StrategyValidator {
                    kind: StrategyParameterKind::SingleString,
                    name: String::from("param-2"),
                    mandatory: true,
                },
            ];

            assert_eq!(
                parameter_exists(&String::from("param-3"), &validators),
                Err(StrategyValidatorError::UnknownParameter(String::from(
                    "param-3"
                )))
            );
        }
    }
}
