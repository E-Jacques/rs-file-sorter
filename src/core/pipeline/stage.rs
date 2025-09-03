use crate::core::pipeline::pipeline_data::PipelineContext;

pub trait PipelineStage<T, E>
where
    E: std::error::Error,
{
    fn execute(&self, context: PipelineContext, data: T) -> Result<T, E>;
}
