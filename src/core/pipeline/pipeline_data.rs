use crate::core::strategy::Strategy;

#[derive(Debug, Default, Clone)]
pub struct PipelineContext {
    strategies: Vec<Box<dyn Strategy>>,
    sort_options: crate::core::options::SortOptions,
    input_dir: String,
    output_dir: String,
}

impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Box<dyn Strategy> {
        self.clone_box()
    }
}

impl PipelineContext {
    pub fn strategies(&self) -> Vec<Box<dyn Strategy>> {
        self.strategies.clone()
    }

    pub fn set_strategies(&mut self, strategies: Vec<Box<dyn Strategy>>) {
        self.strategies = strategies;
    }

    #[cfg(test)]
    pub fn set_options(&mut self, options: crate::core::options::SortOptions) {
        self.sort_options = options;
    }

    pub fn options(&self) -> crate::core::options::SortOptions {
        self.sort_options.clone()
    }

    pub fn input_dir(&self) -> String {
        self.input_dir.clone()
    }

    pub fn output_dir(&self) -> String {
        self.output_dir.clone()
    }

    pub fn new(
        strategies: Vec<Box<dyn Strategy>>,
        sort_options: crate::core::options::SortOptions,
        input_dir: String,
        output_dir: String,
    ) -> Self {
        PipelineContext {
            strategies,
            sort_options,
            input_dir,
            output_dir,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum PipelineData {
    #[default]
    Empty,
    Pause,
    Paths(Vec<std::path::PathBuf>),
    Report(crate::core::report::FullReport),
    Context(PipelineContext),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineDataKind {
    Empty,
    Pause,
    Paths,
    Report,
    Context,
}

#[cfg(test)]
impl PipelineData {
    pub fn kind(&self) -> PipelineDataKind {
        match self {
            PipelineData::Empty => PipelineDataKind::Empty,
            PipelineData::Pause => PipelineDataKind::Pause,
            PipelineData::Paths(_) => PipelineDataKind::Paths,
            PipelineData::Report(_) => PipelineDataKind::Report,
            PipelineData::Context(_) => PipelineDataKind::Context,
        }
    }
}
