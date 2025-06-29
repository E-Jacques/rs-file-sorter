pub trait PipelineStage<T, E>
where
    E: std::error::Error,
{
    fn execute(&self, data: T) -> Result<T, E>;
}
