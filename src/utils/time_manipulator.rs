use std::time::SystemTime;

use chrono::{DateTime, Datelike, Local};

pub fn get_month_number_from_systemtime(system_time: SystemTime) -> u32 {
    // Convert the SystemTime to a DateTime in the local timezone
    let modified_datetime: DateTime<Local> = system_time.into();

    // Extract the month from the broken-down time
    modified_datetime.month()
}

pub fn get_year_number_from_systemtime(system_time: SystemTime) -> i32 {
    // Convert the SystemTime to a DateTime in the local timezone
    let modified_datetime: DateTime<Local> = system_time.into();

    // Extract the month from the broken-down time
    modified_datetime.year()
}

#[cfg(test)]
mod time_mapper_test_module {
    use chrono::{TimeZone, Utc};
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn get_year_number_from_systemtime_tests() {
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_year_number_from_systemtime(system_time), 2014);
    }

    #[test]
    fn get_month_number_from_systemtime_tests_random_month() {
        // Random month
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 7);
    }
    
    #[test]
    fn get_month_number_from_systemtime_tests_january_month() {
        // On the edge months
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 1, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 1);
    }

    #[test]
    fn get_month_number_from_systemtime_tests_december_month() {
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 12, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 12);
    }
}
