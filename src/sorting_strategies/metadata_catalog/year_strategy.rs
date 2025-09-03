use crate::core::{context, parameter, strategy, validation};

#[derive(Clone, Debug)]
pub struct YearStrategy;

impl YearStrategy {
    pub fn new() -> Self {
        YearStrategy
    }
}

impl strategy::Apply for YearStrategy {
    fn apply(&self, _: &std::path::PathBuf, f: &std::fs::File) -> Option<String> {
        match crate::utils::file_manipulator::get_last_modified_time(f) {
            Ok(datetime) => Some(datetime.format("%Y").to_string()),
            Err(error) => panic!("{}", format!("Cannot retrieve year number: {:#?}", error)),
        }
    }
}

impl context::ProcessContext for YearStrategy {
    fn process_context(
        &mut self,
        _: context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        Ok(())
    }
}

impl strategy::Validate for YearStrategy {
    fn validate(&self) -> Result<(), validation::error::Error> {
        Ok(())
    }
}

impl strategy::Name for YearStrategy {
    fn name(&self) -> String {
        "year".to_string()
    }
}

impl strategy::AddParameter for YearStrategy {
    fn add_parameter(&mut self, _: String, _: parameter::StrategyParameter) {}
}

impl strategy::ParameterDetails for YearStrategy {
    fn parameter_details(&self) -> Vec<validation::ParameterDetail> {
        vec![]
    }
}
