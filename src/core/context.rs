use std::path::PathBuf;

use crate::core::error;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StrategyContext {
    files: Vec<PathBuf>,
}

impl StrategyContext {
    pub fn new(files: Vec<PathBuf>) -> Self {
        StrategyContext { files }
    }

    pub fn files(&self) -> Vec<PathBuf> {
        self.files.clone()
    }
}

pub trait ProcessContext {
    fn process_context(&mut self, context: StrategyContext) -> Result<(), error::Error>;
}
