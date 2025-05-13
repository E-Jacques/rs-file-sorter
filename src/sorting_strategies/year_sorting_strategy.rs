use std::fs::File;

use crate::{core::sorting_strategy::SortingStrategy, utils::file_manipulator::get_year_number};

pub fn get_year_sorting_strategy() -> SortingStrategy {
    SortingStrategy {
        name: String::from("year"),
        action: |f: &File| match get_year_number(f) {
            Ok(year_number) => year_number.to_string(),
            Err(error) => panic!("{}", format!("Cannot retrieve year number: {:#?}", error)),
        },
    }
}
