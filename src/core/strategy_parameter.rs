use crate::core::sorting_strategy::SortingStrategy;

#[derive(Clone)]
pub enum StrategyParameter {
    Strategy(Vec<Box<SortingStrategy>>),
    SingleString(String),
}
