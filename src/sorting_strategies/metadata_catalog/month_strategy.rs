use std::str::FromStr;

use crate::core::{context, parameter, strategy, validation};

use super::utils;

static SUPPORTED_LOCALES: &'static [chrono::Locale] = &[
    chrono::Locale::fr_FR,
    chrono::Locale::en_US,
    chrono::Locale::es_ES,
];
const LOCALE_PARAMETER_NAME: &str = "locale";

#[derive(Clone, Debug)]
pub struct MonthStrategy {
    validator: utils::BaseValidator,
    parameters: std::collections::HashMap<String, parameter::StrategyParameter>,
}

impl MonthStrategy {
    pub fn new() -> Self {
        let mut locale_validation_criteria = validation::ParameterDetail::new(
            LOCALE_PARAMETER_NAME,
            parameter::StrategyParameterKind::Choice(
                SUPPORTED_LOCALES
                    .iter()
                    .map(chrono::Locale::to_string)
                    .collect(),
            ),
            false,
        );
        locale_validation_criteria.with_default_value(parameter::StrategyParameter::SingleString(
            chrono::Locale::en_US.to_string(),
        ));

        let mut validator = utils::BaseValidator::new();
        validator.add_validator(locale_validation_criteria);

        let parameters = validator.default_parameters();

        MonthStrategy {
            validator,
            parameters,
        }
    }
}

impl strategy::Apply for MonthStrategy {
    fn apply(&self, _: &std::path::PathBuf, f: &std::fs::File) -> Option<String> {
        match crate::utils::file_manipulator::get_last_modified_time(f) {
            Ok(datetime) => {
                let locale: chrono::Locale =
                    if let Some(parameter::StrategyParameter::SingleString(locale_str)) =
                        self.parameters.get(LOCALE_PARAMETER_NAME)
                    {
                        chrono::Locale::from_str(locale_str).unwrap_or(chrono::Locale::fr_FR)
                    } else {
                        chrono::Locale::fr_FR
                    };

                let formatted = datetime.format_localized("%m_%B", locale).to_string();
                Some(formatted)
            }
            Err(error) => panic!("{}", format!("Cannot retrieve month number: {:#?}", error)),
        }
    }
}

impl strategy::Name for MonthStrategy {
    fn name(&self) -> String {
        "month".to_string()
    }
}

impl strategy::AddParameter for MonthStrategy {
    fn add_parameter(&mut self, key: String, value: parameter::StrategyParameter) {
        self.parameters.insert(key, value);
    }
}

impl strategy::Validate for MonthStrategy {
    fn validate(&self) -> Result<(), validation::error::Error> {
        self.validator.validate(&self.parameters)
    }
}

impl strategy::ParameterDetails for MonthStrategy {
    fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
        self.validator.parameter_details()
    }
}

impl context::ProcessContext for MonthStrategy {
    fn process_context(
        &mut self,
        _: context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}
