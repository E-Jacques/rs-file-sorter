use crate::sorting_strategies::{
    analysis_catalog::get_analysis_catalog, manipulation_catalog::get_manipulation_catalog,
    metadata_catalog::get_metadata_catalog, strategy_catalog::StrategyCatalog,
};

pub fn all_catalog() -> StrategyCatalog {
    get_metadata_catalog()
        .with(&get_manipulation_catalog())
        .with(&get_analysis_catalog())
}
