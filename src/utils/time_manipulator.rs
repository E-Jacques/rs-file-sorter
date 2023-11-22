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