use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::core::error::strategy_validator_error::Error;
use crate::core::strategy_parameter::StrategyParameter;
use crate::core::strategy_validator::{parameter_exists, StrategyValidator};

type SortingStrategyAction =
    fn(&std::path::PathBuf, &File, &HashMap<String, StrategyParameter>) -> String;

#[derive(Clone)]
pub struct SortingStrategy {
    pub action: SortingStrategyAction,
    pub name: String,
    pub parameters: HashMap<String, StrategyParameter>,
    pub validators: Vec<StrategyValidator>,
}

impl std::fmt::Debug for SortingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortingStrategy")
            .field("name", &self.name)
            .finish()
    }
}

impl SortingStrategy {
    pub fn new(name: &str, action: SortingStrategyAction) -> SortingStrategy {
        SortingStrategy {
            action: action,
            name: name.to_string(),
            parameters: HashMap::new(),
            validators: vec![],
        }
    }

    pub fn add_parameter(&mut self, key: String, value: StrategyParameter) {
        self.parameters.insert(key, value);
    }

    pub fn add_validator(&mut self, validator: StrategyValidator) -> &mut Self {
        let validator_already_specified = self
            .validators
            .iter()
            .any(|v: &StrategyValidator| v.name == validator.name);
        if validator_already_specified {
            let validator_name = validator.name;
            panic!("[Sorting Strategy] The name associated with each validator should be unique. '{validator_name}' has been specified twice.");
        } else {
            // Set default value in parameter. Can still be overloaded if required
            if let Some(default_value) = validator.default_value.clone() {
                self.add_parameter(validator.name.clone(), default_value);
            }
            self.validators.push(validator);

            self
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        self.parameters
            .keys()
            .try_for_each(|name| parameter_exists(name, &self.validators))
            .and_then(|_| {
                self.validators
                    .iter()
                    .try_for_each(|validator| validator.validate(&self.parameters))
            })
    }

    pub(crate) fn apply(&self, file_path: &std::path::PathBuf, file: &File) -> String {
        let file_mutex = Arc::new(Mutex::new(file));
        let file_clone = Arc::clone(&file_mutex);
        let file_lock = file_clone.lock().unwrap();
        let result = (self.action)(file_path, &*file_lock, &self.parameters.clone());

        return result;
    }
}

#[cfg(test)]
mod test {
    mod test_sorting_strategy_validate {
        use crate::core::{
            error::strategy_validator_error::Error,
            sorting_strategy::SortingStrategy,
            strategy_parameter::{StrategyParameter, StrategyParameterKind},
            strategy_validator::StrategyValidator,
        };

        #[test]
        fn should_return_unknown_parameter_error_when_one_parameter_not_in_specification() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "known-param",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_parameter(
                String::from("unknown-param"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            sorting_strategy.add_parameter(
                String::from("known-param"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(
                sorting_strategy.validate(),
                Err(Error::UnknownParameter(String::from("unknown-param")))
            )
        }

        #[test]
        fn should_return_type_error_when_parameter_kind_missmatch_with_specification() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-2",
                StrategyParameterKind::Strategy,
                false,
            ));
            sorting_strategy.add_parameter(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            sorting_strategy.add_parameter(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(
                sorting_strategy.validate(),
                Err(Error::TypeError(StrategyValidator::new(
                    "param-2",
                    StrategyParameterKind::Strategy,
                    false,
                )))
            )
        }

        #[test]
        fn should_return_missing_mandatory_parameter_error_when_one_required_parameter_is_missing()
        {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));
            sorting_strategy.add_parameter(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(
                sorting_strategy.validate(),
                Err(Error::MissingMandatoryParameter(StrategyValidator::new(
                    "param-1",
                    StrategyParameterKind::SingleString,
                    true,
                )))
            )
        }

        #[test]
        fn should_return_ok_if_specification_is_respected() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));
            sorting_strategy.add_parameter(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            sorting_strategy.add_parameter(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(sorting_strategy.validate(), Ok(()))
        }
    }

    mod test_add_validator {
        use crate::core::{
            sorting_strategy::SortingStrategy, strategy_parameter::StrategyParameterKind,
            strategy_validator::StrategyValidator,
        };

        #[test]
        #[should_panic(
            expected = "[Sorting Strategy] The name associated with each validator should be unique. 'param-twice' has been specified twice."
        )]
        fn should_panic_if_validator_with_same_name_specified_twice() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-twice",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-twice",
                StrategyParameterKind::SingleString,
                false,
            ));
        }

        #[test]
        fn should_push_to_the_list_of_validators() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            sorting_strategy.add_validator(StrategyValidator::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));

            assert_eq!(
                sorting_strategy.validators,
                vec![
                    StrategyValidator::new("param-1", StrategyParameterKind::SingleString, true),
                    StrategyValidator::new("param-2", StrategyParameterKind::SingleString, false)
                ]
            )
        }
    }

    mod test_add_parameter {
        use std::collections::HashMap;

        use crate::core::{
            sorting_strategy::SortingStrategy, strategy_parameter::StrategyParameter,
        };

        #[test]
        fn should_push_to_the_hashmap_of_parameters() {
            let mut sorting_strategy =
                SortingStrategy::new("my-strategy", |_, _, _| String::default());
            sorting_strategy.add_parameter(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            sorting_strategy.add_parameter(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            let mut expected_parameters = HashMap::new();
            expected_parameters.insert(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            expected_parameters.insert(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );
            assert_eq!(sorting_strategy.parameters, expected_parameters)
        }
    }
}
