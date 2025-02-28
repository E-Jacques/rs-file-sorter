use crate::sorting_strategies::{
    month_sorting_strategy::get_month_sorting_strategy,
    year_sorting_strategy::get_year_sorting_strategy,
};

use super::sorting_strategy::SortingStrategy;

pub fn get_storting_strategies_list() -> Vec<SortingStrategy> {
    vec![get_month_sorting_strategy(), get_year_sorting_strategy()]
}
