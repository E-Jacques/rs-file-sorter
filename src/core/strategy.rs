use crate::core::context::ProcessContext;
use crate::core::parameter::StrategyParameter;
use crate::core::validation;
use std::fs::File;

pub trait StrategyCloneBox {
    fn clone_box(&self) -> Box<dyn Strategy>;
}
impl<T> StrategyCloneBox for T
where
    T: 'static + Strategy + Clone,
{
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

pub trait Strategy:
    Apply
    + Validate
    + ProcessContext
    + AddParameter
    + ParameterDetails
    + Parameters
    + Name
    + StrategyCloneBox
{
    fn as_apply(&self) -> Box<dyn Apply>;
    fn as_validate(&self) -> Box<dyn Validate>;
    fn as_process(&self) -> Box<dyn ProcessContext>;
}

impl<
        T: Apply
            + Validate
            + ProcessContext
            + AddParameter
            + ParameterDetails
            + Parameters
            + Name
            + Clone
            + 'static,
    > Strategy for T
{
    fn as_apply(&self) -> Box<dyn Apply> {
        Box::new(self.clone())
    }

    fn as_validate(&self) -> Box<dyn Validate> {
        Box::new(self.clone())
    }

    fn as_process(&self) -> Box<dyn ProcessContext> {
        Box::new(self.clone())
    }
}

// The main trait
pub trait Apply: std::fmt::Debug {
    fn apply(&self, file_path: &std::path::PathBuf, file: &File) -> Option<String>;
}

pub trait Name {
    fn name(&self) -> String;
}

pub trait AddParameter {
    fn add_parameter(&mut self, key: String, value: StrategyParameter);
}

pub trait ParameterDetails {
    fn parameter_details(&self) -> Vec<validation::ParameterDetail>;
}

pub trait Validate {
    fn validate(&self) -> Result<(), validation::error::Error>;
}

pub trait AsStrategy {
    fn as_strategy(&self) -> &dyn Strategy;
}

pub trait Parameters {
    fn parameters(&self) -> std::collections::HashMap<String, StrategyParameter>;
}
