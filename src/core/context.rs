use std::path::PathBuf;

use crate::core::error;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StrategyContext {
    pub files: Vec<PathBuf>,
}

pub trait ProcessContext {
    fn process_context(&mut self, context: StrategyContext) -> Result<(), error::Error>;
}
