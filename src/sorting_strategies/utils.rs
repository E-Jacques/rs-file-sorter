#[derive(Clone, Debug)]
pub struct BaseValidator {
    validators: Vec<crate::core::validation::ParameterDetail>,
}

impl BaseValidator {
    pub fn new() -> BaseValidator {
        BaseValidator {
            validators: Vec::new(),
        }
    }

    pub fn add_validator(
        &mut self,
        validator: crate::core::validation::ParameterDetail,
    ) -> &mut Self {
        let validator_already_specified = self
            .validators
            .iter()
            .any(|v: &crate::core::validation::ParameterDetail| v.name == validator.name);
        if validator_already_specified {
            let validator_name = validator.name;
            panic!("[Sorting Strategy] The name associated with each validator should be unique. '{validator_name}' has been specified twice.");
        } else {
            self.validators.push(validator);

            self
        }
    }

    pub fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        self.validators.clone()
    }

    pub fn default_parameters(
        &self,
    ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter> {
        self.validators
            .iter()
            .filter_map(|v| {
                v.default_value
                    .as_ref()
                    .map(|default_value| (v.name.clone(), default_value.clone()))
            })
            .collect()
    }

    pub fn validate(
        &self,
        parameters: &std::collections::HashMap<String, crate::core::parameter::StrategyParameter>,
    ) -> Result<(), crate::core::validation::error::Error> {
        parameters
            .keys()
            .try_for_each(|name| crate::core::validation::parameter_exists(name, &self.validators))
            .and_then(|_| {
                self.validators
                    .iter()
                    .try_for_each(|validator| validator.validate(parameters))
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_sorting_strategy_validate {
        use crate::{
            core::{
                parameter::{StrategyParameter, StrategyParameterKind},
                validation::{self, error::Error},
            },
            sorting_strategies::utils::BaseValidator,
        };

        #[test]
        fn should_return_unknown_parameter_error_when_one_parameter_not_in_specification() {
            let mut validator = BaseValidator::new();
            validator.add_validator(validation::ParameterDetail::new(
                "known-param",
                StrategyParameterKind::SingleString,
                true,
            ));

            let mut params = std::collections::HashMap::new();
            params.insert(
                String::from("known-param"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            params.insert(
                String::from("unknown-param"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(
                validator.validate(&params),
                Err(Error::UnknownParameter(String::from("unknown-param")))
            )
        }

        #[test]
        fn should_return_type_error_when_parameter_kind_missmatch_with_specification() {
            let mut validator = BaseValidator::new();
            validator.add_validator(validation::ParameterDetail::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            validator.add_validator(validation::ParameterDetail::new(
                "param-2",
                StrategyParameterKind::Strategy,
                false,
            ));

            let mut params = std::collections::HashMap::new();
            params.insert(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            params.insert(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(
                validator.validate(&params),
                Err(Error::TypeError(validation::ParameterDetail::new(
                    "param-2",
                    StrategyParameterKind::Strategy,
                    false,
                )))
            )
        }

        #[test]
        fn should_return_missing_mandatory_parameter_error_when_one_required_parameter_is_missing()
        {
            let mut validator = BaseValidator::new();
            validator.add_validator(validation::ParameterDetail::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            validator.add_validator(validation::ParameterDetail::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));

            let params = std::collections::HashMap::from([(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            )]);

            assert_eq!(
                validator.validate(&params),
                Err(Error::MissingMandatoryParameter(
                    validation::ParameterDetail::new(
                        "param-1",
                        StrategyParameterKind::SingleString,
                        true,
                    )
                ))
            )
        }

        #[test]
        fn should_return_ok_if_specification_is_respected() {
            let mut validator = BaseValidator::new();
            validator.add_validator(validation::ParameterDetail::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            validator.add_validator(validation::ParameterDetail::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));

            let mut params = std::collections::HashMap::new();
            params.insert(
                String::from("param-1"),
                StrategyParameter::SingleString(String::from("My value 1")),
            );
            params.insert(
                String::from("param-2"),
                StrategyParameter::SingleString(String::from("My value 2")),
            );

            assert_eq!(validator.validate(&params), Ok(()))
        }
    }

    mod test_add_validator {
        use super::*;
        use crate::core::parameter::StrategyParameterKind;

        #[test]
        #[should_panic(
            expected = "[Sorting Strategy] The name associated with each validator should be unique. 'param-twice' has been specified twice."
        )]
        fn should_panic_if_validator_with_same_name_specified_twice() {
            let mut validator = BaseValidator::new();
            validator.add_validator(crate::core::validation::ParameterDetail::new(
                "param-twice",
                StrategyParameterKind::SingleString,
                true,
            ));
            validator.add_validator(crate::core::validation::ParameterDetail::new(
                "param-twice",
                StrategyParameterKind::SingleString,
                false,
            ));
        }

        #[test]
        fn should_push_to_the_list_of_validators() {
            let mut validator = BaseValidator::new();
            validator.add_validator(crate::core::validation::ParameterDetail::new(
                "param-1",
                StrategyParameterKind::SingleString,
                true,
            ));
            validator.add_validator(crate::core::validation::ParameterDetail::new(
                "param-2",
                StrategyParameterKind::SingleString,
                false,
            ));

            assert_eq!(
                validator.parameter_details(),
                vec![
                    crate::core::validation::ParameterDetail::new(
                        "param-1",
                        StrategyParameterKind::SingleString,
                        true
                    ),
                    crate::core::validation::ParameterDetail::new(
                        "param-2",
                        StrategyParameterKind::SingleString,
                        false
                    )
                ]
            )
        }
    }
}
