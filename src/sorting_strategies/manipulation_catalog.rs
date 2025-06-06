use std::{collections::HashMap, fs::File};

use crate::{
    core::{
        sorting_strategy::SortingStrategy,
        strategy_parameter::{StrategyParameter, StrategyParameterKind},
        strategy_validator::StrategyValidator,
    },
    sorting_strategies::strategy_catalog::StrategyCatalog,
};

pub fn get_manipulation_catalog() -> StrategyCatalog {
    StrategyCatalog::from(vec![get_concat_strategy(), get_text_strategy()])
}

fn get_concat_strategy() -> SortingStrategy {
    let mut strategy = SortingStrategy::new(
        "concat",
        move |f: &File, parameters: &HashMap<String, StrategyParameter>| {
            let mut result = String::new();

            let strategies = parameters.get("strategies").unwrap();
            match strategies {
                StrategyParameter::Strategy(strategies) => {
                    for strategy in strategies {
                        let action = &strategy.action;
                        result.push_str(&action(f, &strategy.parameters));
                    }
                }
                _ => (),
            }
            result
        },
    );
    strategy.add_validator(StrategyValidator::new(
        "strategies",
        StrategyParameterKind::Strategy,
        true,
    ));

    strategy
}

fn get_text_strategy() -> SortingStrategy {
    let mut strategy = SortingStrategy::new(
        "text",
        move |_: &File, parameters: &HashMap<String, StrategyParameter>| {
            let mut result = String::new();

            let strategies = parameters.get("value").unwrap();
            match strategies {
                StrategyParameter::SingleString(value) => {
                    result = value.clone();
                }
                _ => (),
            }
            result
        },
    );
    strategy.add_validator(StrategyValidator::new(
        "value",
        StrategyParameterKind::SingleString,
        true,
    ));

    strategy
}
