use std::{
    fs::File,
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::utils::{string_manipulator::add_0_to_single_number, time_manipulator};

use super::sorting_strategy::SortingStrategy;

fn get_month_number(file: &File) -> Result<u32, io::Error> {
    // Get the metadata of the file
    let metadata = file.metadata()?;

    // Extract the modification time from the metadata
    let modified_time = metadata.modified()?;

    // Convert the modification time (seconds since UNIX_EPOCH) into a SystemTime
    let system_time = SystemTime::UNIX_EPOCH + modified_time.duration_since(UNIX_EPOCH).unwrap();

    // Extract the month from the broken-down time
    let month = time_manipulator::get_month_number_from_systemtime(system_time);

    Ok(month)
}

fn get_year_number(file: &File) -> Result<i32, io::Error> {
    // Get the metadata of the file
    let metadata = file.metadata()?;

    // Extract the modification time from the metadata
    let modified_time = metadata.modified()?;

    // Convert the modification time (seconds since UNIX_EPOCH) into a SystemTime
    let system_time = SystemTime::UNIX_EPOCH + modified_time.duration_since(UNIX_EPOCH).unwrap();

    let year = time_manipulator::get_year_number_from_systemtime(system_time);

    Ok(year)
}

#[test]
fn get_month_number_test() {
    let file = File::open(std::path::Path::new("tests/rsc/files/test_20_02_2022"))
        .expect("Unable to locate the \'test/rsc/files/test_20_02_2022\' file.");
    assert_eq!(
        get_month_number(&file).expect("Should't receive error when trying to read this file."),
        2
    );
}

#[test]
fn get_year_number_test() {
    let file = File::open(std::path::Path::new("tests/rsc/files/test_20_02_2022"))
        .expect("Unable to locate the \'test/rsc/files/test_20_02_2022\' file.");

    assert_eq!(
        get_year_number(&file).expect("Should't receive error when trying to read this file."),
        2022
    );
}

pub fn get_year_month_sorting_strategy() -> SortingStrategy {
    let mut year_month_placement = SortingStrategy::new();

    year_month_placement.set_method_chain(vec![
        |file: &File| get_year_number(file).unwrap().to_string(),
        |file: &File| {
            let month_names = vec![
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

            let month = get_month_number(file).unwrap();
            let month_index: usize = month.try_into().unwrap();
            let month_name: &str = month_names[month_index - 1];
            format!("{}_{}", add_0_to_single_number(month), month_name)
        },
    ]);

    year_month_placement
}

#[cfg(test)]
pub mod year_month_sorting_strategy_test_module {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_with_real_file() {
        let file = File::open(std::path::Path::new("tests/rsc/files/test_20_02_2022"))
            .expect("Unable to locate the \'test/rsc/files/test_20_02_2022\' file.");
        let placement_strategy = get_year_month_sorting_strategy();
        let mut iterator = placement_strategy.iter(file);
        let first_value = iterator.next();
        match first_value {
            Some(val) => assert_eq!(val, String::from("2022")),
            None => assert!(false),
        }

        let second_value = iterator.next();
        match second_value {
            Some(val) => assert_eq!(val, String::from("02_Février")),
            None => assert!(false),
        }

        let second_value = iterator.next();
        match second_value {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
