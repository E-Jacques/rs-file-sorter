use std::fmt::Display;

use crate::core::sorting_strategy::SortingStrategy;

#[derive(Clone, Debug, PartialEq)]
pub enum StrategyParameter {
    Strategy(Vec<Box<SortingStrategy>>),
    SingleString(String),
}

impl PartialEq for Box<SortingStrategy> {
    fn eq(&self, other: &Self) -> bool {
        *self.name == other.name && self.parameters == other.parameters
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum StrategyParameterKind {
    Strategy,
    SingleString,
    Choice(Vec<String>),
}

impl Display for StrategyParameterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            StrategyParameterKind::Strategy => "strategy",
            StrategyParameterKind::SingleString => "single string",
            StrategyParameterKind::Choice(choices) => {
                let list_str = choices.join(",");
                return write!(f, "choice: ({})", list_str);
            }
        })
    }
}

impl StrategyParameter {
    pub fn kind(&self) -> StrategyParameterKind {
        match self {
            StrategyParameter::SingleString(_) => StrategyParameterKind::SingleString,
            StrategyParameter::Strategy(_) => StrategyParameterKind::Strategy,
        }
    }
}
