use std::fs::File;

use crate::core::sorting_strategy::SortingStrategy;

pub fn concat_strategy(strategies: Vec<SortingStrategy>) -> SortingStrategy {
    let borrowed_strategies: Vec<SortingStrategy> = strategies.clone();
    SortingStrategy::new(
        "concat",
        Box::new(move |f: &File| {
            let mut result = String::new();
            for strategy in &borrowed_strategies {
                let action = &strategy.action;
                result.push_str(&action(f));
            }
            result
        }),
    )
}
