use crate::sorting_strategies::strategy_catalog::StrategyCatalog;

mod text_semantic_strategy;

pub fn get_analysis_catalog() -> StrategyCatalog {
    StrategyCatalog::new(vec![Box::new(
        text_semantic_strategy::TextSemanticStrategy::new(),
    )])
}
