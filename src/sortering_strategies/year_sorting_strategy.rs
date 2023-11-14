use std::fs::File;

use crate::utils::file_manipulator::get_year_number;

use super::sorting_strategy::SortingStrategy;

pub fn get_year_sorting_strategy() -> SortingStrategy {
    SortingStrategy {
        name: String::from("year"),
        action: |f: &File| match get_year_number(f) {
            Ok(year_number) => year_number.to_string(),
            Err(error) => panic!("{}", format!("Cannot retrieve year number: {:#?}", error)),
        },
    }
}
