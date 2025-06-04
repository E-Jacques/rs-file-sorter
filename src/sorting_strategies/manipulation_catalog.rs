use std::{collections::HashMap, fs::File};

use crate::{
    core::sorting_strategy::{SortingStrategy, StrategyParameter},
    sorting_strategies::strategy_catalog::StrategyCatalog,
};

pub fn get_manipulation_catalog() -> StrategyCatalog {
    StrategyCatalog::from(vec![get_concat_strategy()])
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
            }
            result
        },
    )
}
