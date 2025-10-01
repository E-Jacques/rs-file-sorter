// TODO : Paramétrer le nombre de cluster. Créer un input number. Améliorer le fonctionnement de KMean avec des strategies.

use crate::{
    core::{
        parameter::{self, StrategyParameter},
        strategy, validation,
    },
    sorting_strategies::utils,
    utils::{
        cluster::{k_mean, point},
        nlp,
    },
};

const DEFAULT_CLUSTER_NUMBER: usize = 5;
const TF_IDF_ITERATION: usize = 400;
const CLUSTER_NUMBER_PARAM_NAME: &str = "cluster.number";

#[derive(Debug, Clone)]
pub struct TextSemanticStrategy {
    validator: utils::BaseValidator,
    parameters: std::collections::HashMap<String, parameter::StrategyParameter>,
    context: std::collections::HashMap<std::path::PathBuf, String>,
}

impl TextSemanticStrategy {
    pub fn new() -> Self {
        let mut cluster_number_param = validation::ParameterDetail::new(
            CLUSTER_NUMBER_PARAM_NAME,
            parameter::StrategyParameterKind::Number,
            true,
        );
        cluster_number_param.with_default_value(StrategyParameter::Number(DEFAULT_CLUSTER_NUMBER));

        let mut validator = utils::BaseValidator::new();
        validator.add_validator(cluster_number_param);

        let parameters = validator.default_parameters();
        TextSemanticStrategy {
            validator,
            parameters,
            context: std::collections::HashMap::new(),
        }
    }
}

impl strategy::Apply for TextSemanticStrategy {
    fn apply(&self, file_path: &std::path::PathBuf, _: &std::fs::File) -> Option<String> {
        self.context.get(file_path).cloned()
    }
}

impl strategy::Validate for TextSemanticStrategy {
    fn validate(&self) -> Result<(), crate::core::validation::error::Error> {
        self.validator.validate(&self.parameters)
    }
}

impl strategy::Name for TextSemanticStrategy {
    fn name(&self) -> String {
        "text semantic".to_string()
    }
}

impl strategy::AddParameter for TextSemanticStrategy {
    fn add_parameter(&mut self, name: String, parameter: parameter::StrategyParameter) {
        self.parameters.insert(name, parameter);
    }
}

impl strategy::ParameterDetails for TextSemanticStrategy {
    fn parameter_details(&self) -> Vec<crate::core::validation::ParameterDetail> {
        self.validator.parameter_details()
    }
}

/// Find the index of the maximum value in a vector
///
/// Returns `None` if the vector is empty
fn find_index_of_max_value(values: &Vec<f32>) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    let mut max_index = 0;
    let mut max_value = values[0];
    for (i, &value) in values.iter().enumerate().skip(1) {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }
    Some(max_index)
}

impl crate::core::context::ProcessContext for TextSemanticStrategy {
    fn process_context(
        &mut self,
        strategy_context: crate::core::context::StrategyContext,
    ) -> Result<(), crate::core::error::Error> {
        // Reset the actual strategy context
        self.context = std::collections::HashMap::new();

        // Retrieve the content of all files
        let contents: Vec<String> = strategy_context
            .files()
            .iter()
            .filter_map(|file| std::fs::read_to_string(file.as_path()).ok())
            .collect::<Vec<String>>();

        // Compute the tf-idf for all files
        let tf_idf: nlp::TfIdf = nlp::tf_idf(contents);

        // Cluster the files based on their tf-idf values
        let cluster_number = self
            .parameters
            .get(CLUSTER_NUMBER_PARAM_NAME)
            .and_then(|param| {
                if let StrategyParameter::Number(n) = param {
                    Some(*n)
                } else {
                    None
                }
            })
            .unwrap_or(DEFAULT_CLUSTER_NUMBER);
        let kmean = k_mean::k_mean(
            tf_idf
                .tf_idf()
                .iter()
                .cloned()
                .map(point::Point::new)
                .collect(),
            cluster_number,
            TF_IDF_ITERATION,
        )
        .map_err(|err| crate::core::error::Error::Strategy(err.kind().to_string()))?;

        // Associate each file to the most meaningfull word in its cluster
        for i in 0..kmean.labels.len() {
            let file_path = &strategy_context.files()[i];
            let cluster_index = kmean.labels[i];
            let most_meaningfull_cluster_vector =
                find_index_of_max_value(&kmean.centroids[cluster_index].get())
                    .expect("vector should be empty");
            let group_name = &tf_idf.terms()[most_meaningfull_cluster_vector];
            self.context
                .insert(file_path.clone(), group_name.to_string());
        }

        Ok(())
    }
}
