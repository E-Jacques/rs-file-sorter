mod concat_strategy;
mod or_strategy;
mod text_strategy;

use crate::sorting_strategies::strategy_catalog::StrategyCatalog;

pub fn get_manipulation_catalog() -> StrategyCatalog {
    StrategyCatalog::new(vec![
        Box::new(concat_strategy::ConcatStrategy::new()),
        Box::new(text_strategy::TextStrategy::new()),
        Box::new(or_strategy::OrStrategy::new()),
    ])
}
