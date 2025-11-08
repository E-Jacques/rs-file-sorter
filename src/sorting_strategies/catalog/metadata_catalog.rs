use super::strategy_catalog::StrategyCatalog;
mod file_ext_strategy;
mod file_type_strategy;
mod month_strategy;
mod year_strategy;

pub fn get_metadata_catalog() -> StrategyCatalog {
    StrategyCatalog::new(vec![
        Box::new(month_strategy::MonthStrategy::new()),
        Box::new(year_strategy::YearStrategy::new()),
        Box::new(file_ext_strategy::FileExtStrategy::new()),
        Box::new(file_type_strategy::FileTypeStrategy::new()),
    ])
}
