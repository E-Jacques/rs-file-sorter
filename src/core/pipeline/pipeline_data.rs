#[derive(Debug, Default, Clone)]
pub enum PipelineData {
    #[default]
    Empty,
    Pause,
    Paths(Vec<std::path::PathBuf>),
    Report(crate::core::sorter::FullSorterReport),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineDataKind {
    Empty,
    Pause,
    Paths,
    Report,
}

#[cfg(test)]
impl PipelineData {
    pub fn kind(&self) -> PipelineDataKind {
        match self {
            PipelineData::Empty => PipelineDataKind::Empty,
            PipelineData::Pause => PipelineDataKind::Pause,
            PipelineData::Paths(_) => PipelineDataKind::Paths,
            PipelineData::Report(_) => PipelineDataKind::Report,
        }
    }
}
