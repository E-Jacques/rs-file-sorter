use std::{collections::HashMap, fs::File};

use crate::{
    core::sorting_strategy::{SortingStrategy, StrategyParameter},
    sorting_strategies::strategy_catalog::StrategyCatalog,
};

pub fn get_manipulation_catalog() -> StrategyCatalog {
    StrategyCatalog::from(vec![get_concat_strategy(), get_text_strategy()])
}

fn get_concat_strategy() -> SortingStrategy {
    SortingStrategy::new(
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
    )
}

fn get_text_strategy() -> SortingStrategy {
    SortingStrategy::new(
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
    )
}
