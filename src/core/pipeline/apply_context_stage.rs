use crate::core::{
    context::StrategyContext,
    error,
    pipeline::{
        pipeline_data::{PipelineContext, PipelineData},
        stage::PipelineStage,
    },
};

pub struct ApplyContextStage;
impl PipelineStage<PipelineData, error::Error> for ApplyContextStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        let files = match &data {
            PipelineData::Paths(paths) => paths.clone(),
            _ => return Err(error::Error::Pipeline),
        };
        let strategy_context = StrategyContext::new(files);
        let mut new_context = context.clone();
        let new_strategies = new_context
            .strategies()
            .iter()
            .map(|processor| {
                let mut p = processor.clone();
                p.process_context(strategy_context.clone())?;
                Ok(p)
            })
            .collect::<Result<Vec<_>, error::Error>>()?;
        new_context.set_strategies(new_strategies);

        Ok(PipelineData::Context(new_context))
    }
}

impl std::fmt::Display for ApplyContextStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplyContextStage")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::{
        context,
        pipeline::pipeline_data::{PipelineContext, PipelineDataKind},
        strategy::{AddParameter, Apply, Name, ParameterDetails, Parameters, Validate},
        validation,
    };

    use super::*;

    #[derive(Default, Clone, Debug)]
    struct TestContext {
        context: StrategyContext,
    }
    impl context::ProcessContext for TestContext {
        fn process_context(&mut self, context: StrategyContext) -> Result<(), error::Error> {
            self.context = context;
            Ok(())
        }
    }

    impl Validate for TestContext {
        fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
            Ok(())
        }
    }

    impl Apply for TestContext {
        fn apply(&self, _: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
            None
        }
    }

    impl AddParameter for TestContext {
        fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
    }

    impl ParameterDetails for TestContext {
        fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
            vec![]
        }
    }

    impl Name for TestContext {
        fn name(&self) -> String {
            "TestContext".to_string()
        }
    }

    impl Parameters for TestContext {
        fn parameters(
            &self,
        ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter> {
            std::collections::HashMap::new()
        }
    }

    #[derive(Debug, Clone)]
    struct TestErrorContext;
    impl context::ProcessContext for TestErrorContext {
        fn process_context(&mut self, _: StrategyContext) -> Result<(), error::Error> {
            Err(error::Error::Strategy(
                "Failed processing context".to_string(),
            ))
        }
    }

    impl Validate for TestErrorContext {
        fn validate(&self) -> Result<(), validation::error::Error> {
            Ok(())
        }
    }

    impl Apply for TestErrorContext {
        fn apply(&self, _: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
            None
        }
    }

    impl AddParameter for TestErrorContext {
        fn add_parameter(&mut self, _: String, _: crate::core::parameter::StrategyParameter) {}
    }

    impl ParameterDetails for TestErrorContext {
        fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
            vec![]
        }
    }

    impl Name for TestErrorContext {
        fn name(&self) -> String {
            "TestErrorContext".to_string()
        }
    }

    impl Parameters for TestErrorContext {
        fn parameters(
            &self,
        ) -> std::collections::HashMap<String, crate::core::parameter::StrategyParameter> {
            std::collections::HashMap::new()
        }
    }

    #[test]
    fn test_should_return_type_error_for_non_files_data() {
        let data = PipelineData::Empty;
        let context = PipelineContext::new(
            vec![
                Box::new(TestContext::default()),
                Box::new(TestContext::default()),
            ],
            crate::core::options::SortOptions::default(),
            "input_dir".to_string(),
            "output_dir".to_string(),
        );

        let result = ApplyContextStage.execute(context, data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), error::ErrorKind::Pipeline);
    }

    #[test]
    fn test_should_pass_overall_context_to_context_processors() {
        let data = PipelineData::Paths(vec![PathBuf::new()]);
        let pipeline_context = PipelineContext::new(
            vec![
                Box::new(TestContext::default()),
                Box::new(TestContext::default()),
            ],
            crate::core::options::SortOptions::default(),
            "input_dir".to_string(),
            "output_dir".to_string(),
        );

        let result = ApplyContextStage.execute(pipeline_context, data);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap().kind(), PipelineDataKind::Context);
    }

    #[test]
    fn test_should_forward_any_error() {
        let data = PipelineData::Paths(vec![PathBuf::new()]);
        let pipeline_context = PipelineContext::new(
            vec![Box::new(TestErrorContext)],
            crate::core::options::SortOptions::default(),
            "input_dir".to_string(),
            "output_dir".to_string(),
        );

        let result = ApplyContextStage.execute(pipeline_context, data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), error::ErrorKind::Strategy);
    }
}
