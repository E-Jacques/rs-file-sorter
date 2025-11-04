use serde::{Deserialize, Serialize};

use crate::{
    core::parameter::StrategyParameter,
    sorting_strategies::{
        analysis_catalog::get_analysis_catalog, manipulation_catalog::get_manipulation_catalog,
        metadata_catalog::get_metadata_catalog,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPayload {
    pub strategy_name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterValue {
    String(String),
    Integer(usize),
    Float(f64),
    Boolean(bool),
    Strategy(StrategyPayload),
    Array(Vec<ParameterValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
}

impl From<StrategyParameter> for ParameterValue {
    fn from(param: StrategyParameter) -> Self {
        match param {
            StrategyParameter::Strategy(value) => ParameterValue::Array(
                value
                    .iter()
                    .map(|s| {
                        ParameterValue::Strategy(StrategyPayload {
                            strategy_name: s.name(),
                            parameters: s
                                .parameters()
                                .iter()
                                .map(|(k, v)| Parameter {
                                    name: k.clone(),
                                    value: v.clone().into(),
                                })
                                .collect(),
                        })
                    })
                    .collect(),
            ),
            StrategyParameter::SingleString(value) => ParameterValue::String(value),
            StrategyParameter::Number(value) => ParameterValue::Integer(value),
        }
    }
}

impl Into<StrategyParameter> for ParameterValue {
    fn into(self) -> StrategyParameter {
        match self {
            ParameterValue::String(value) => StrategyParameter::SingleString(value),
            ParameterValue::Integer(value) => StrategyParameter::Number(value),
            ParameterValue::Array(values) => {
                let mut strategies = vec![];

                for val in values {
                    if let ParameterValue::Strategy(s) = val {
                        let maybe_strategy = get_metadata_catalog()
                            .with(&get_manipulation_catalog())
                            .with(&get_analysis_catalog())
                            .get_strategy(&s.strategy_name);

                        if let Some(strategy) = maybe_strategy {
                            strategies.push(strategy);
                        }
                    }
                }

                StrategyParameter::Strategy(strategies)
            }
            _ => panic!("Unsupported ParameterValue to StrategyParameter conversion"),
        }
    }
}
