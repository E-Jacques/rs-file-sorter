use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub strategies: Vec<super::strategy_payload::StrategyPayload>,
    pub input: String,
    pub output: String,
    pub options: crate::core::options::SortOptions,
}
