#[derive(Debug, Clone)]
pub struct SortPayload {
    pub input: String,
    pub output: String,
    pub strategies: Vec<Box<dyn crate::core::strategy::Strategy>>,
    pub options: crate::core::options::SortOptions,
}
