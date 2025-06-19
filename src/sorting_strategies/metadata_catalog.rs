use std::{collections::HashMap, fs::File, str::FromStr};

use crate::{
    core::{
        sorting_strategy::SortingStrategy,
        strategy_parameter::{StrategyParameter, StrategyParameterKind},
        strategy_validator::StrategyValidator,
    },
    sorting_strategies::strategy_catalog::StrategyCatalog,
    utils::file_manipulator::get_last_modified_time,
};

static SUPPORTED_LOCALES: &'static [chrono::Locale] = &[
    chrono::Locale::fr_FR,
    chrono::Locale::en_US,
    chrono::Locale::es_ES,
];
const LOCALE_PARAMETER_NAME: &str = "locale";

pub fn get_metadata_catalog() -> StrategyCatalog {
    StrategyCatalog::new(vec![
        get_month_sorting_strategy(),
        get_year_sorting_strategy(),
    ])
}

fn get_month_sorting_strategy() -> SortingStrategy {
    let mut strategy = SortingStrategy::new(
        "month",
        |f: &File, parameters: &HashMap<String, StrategyParameter>| match get_last_modified_time(f)
        {
            Ok(datetime) => {
                let locale: chrono::Locale =
                    if let Some(StrategyParameter::SingleString(locale_str)) =
                        parameters.get(LOCALE_PARAMETER_NAME)
                    {
                        chrono::Locale::from_str(locale_str).unwrap_or(chrono::Locale::fr_FR)
                    } else {
                        chrono::Locale::fr_FR
                    };

                datetime.format_localized("%m_%B", locale).to_string()
            }
            Err(error) => panic!("{}", format!("Cannot retrieve month number: {:#?}", error)),
        },
    );
    let mut locale_validator = StrategyValidator::new(
        LOCALE_PARAMETER_NAME,
        StrategyParameterKind::Choice(
            SUPPORTED_LOCALES
                .iter()
                .map(chrono::Locale::to_string)
                .collect(),
        ),
        true,
    );
    locale_validator.with_default_value(StrategyParameter::SingleString(
        chrono::Locale::en_US.to_string(),
    ));
    strategy.add_validator(locale_validator);

    strategy
}

fn get_year_sorting_strategy() -> SortingStrategy {
    SortingStrategy::new("year", |f: &File, _| match get_last_modified_time(f) {
        Ok(datetime) => datetime.format("%Y").to_string(),
        Err(error) => panic!("{}", format!("Cannot retrieve year number: {:#?}", error)),
    })
}
