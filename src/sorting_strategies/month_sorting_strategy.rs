use std::fs::File;

use crate::utils::{
    file_manipulator::get_month_number, string_manipulator::add_0_to_single_number,
};

use super::sorting_strategy::SortingStrategy;

pub fn get_month_sorting_strategy() -> SortingStrategy {
    SortingStrategy {
        name: String::from("month"),
        action: |f: &File| match get_month_number(f) {
            Ok(month_number) => {
                let french_month_name = vec![
                    "Janvier",
                    "Février",
                    "Mars",
                    "Avril",
                    "Mai",
                    "Juin",
                    "Juillet",
                    "Août",
                    "Septembre",
                    "Octobre",
                    "Novembre",
                    "Décembre",
                ];

                let vec_index: usize = (month_number - 1).try_into().unwrap();
                format!(
                    "{}_{}",
                    add_0_to_single_number(month_number),
                    french_month_name
                        .get(vec_index)
                        .expect("The month of the file shouldn't exceed 12 !")
                )
            }
            Err(error) => panic!("{}", format!("Cannot retrieve month number: {:#?}", error)),
        },
    }
}
