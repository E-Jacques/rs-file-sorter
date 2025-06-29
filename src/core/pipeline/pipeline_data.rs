#[derive(Debug, Default, Clone)]
pub enum PipelineData {
    #[default]
    Empty,
    Pause,
    Paths(Vec<std::path::PathBuf>),
    Report(crate::core::sorter::FullSorterReport),
}
