use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum StrategyParameter {
    Strategy(Vec<Box<dyn super::strategy::Strategy>>),
    SingleString(String),
    Number(usize),
}

impl PartialEq for StrategyParameter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StrategyParameter::SingleString(s1), StrategyParameter::SingleString(s2)) => s1 == s2,
            (StrategyParameter::Strategy(v1), StrategyParameter::Strategy(v2)) => {
                v1.len() == v2.len()
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum StrategyParameterKind {
    Strategy,
    SingleString,
    Number,
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
            StrategyParameterKind::Number => "number",
        })
    }
}

impl StrategyParameterKind {
    /// Check if the given value matches the kind.
    pub fn is_matching(&self, value: &StrategyParameter) -> bool {
        match self {
            StrategyParameterKind::Strategy
            | StrategyParameterKind::SingleString
            | StrategyParameterKind::Number => value.kind() == *self,
            StrategyParameterKind::Choice(items) => {
                if let StrategyParameter::SingleString(single_string) = value {
                    items.iter().any(|item| item == single_string)
                } else {
                    false
                }
            }
        }
    }
}

impl StrategyParameter {
    pub fn kind(&self) -> StrategyParameterKind {
        match self {
            StrategyParameter::SingleString(_) => StrategyParameterKind::SingleString,
            StrategyParameter::Strategy(_) => StrategyParameterKind::Strategy,
            StrategyParameter::Number(_) => StrategyParameterKind::Number,
        }
    }
}
