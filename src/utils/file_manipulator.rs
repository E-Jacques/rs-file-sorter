use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, io};

use super::time_manipulator;

pub fn to_absolute_path(path: String) -> String {
    let current_directory = &env::current_dir().expect("[Sort Command] An internal error occured.");
    let path_as_path = Path::new(&path);
    if Path::is_absolute(path_as_path) {
        path
    } else {
        current_directory
            .clone()
            .join(path_as_path)
            .as_os_str()
            .to_str()
            .expect("An internal error occured.")
            .to_string()
    }
}

pub fn to_relative_path(path: String) -> String {
    let current_directory = &env::current_dir().expect("[Sort Command] An internal error occured.");
    let path_as_path = Path::new(&path);
    if Path::is_relative(path_as_path) {
        path
    } else {
        match path_as_path.strip_prefix(current_directory) {
            Ok(relative_path) => relative_path.as_os_str().to_str().unwrap().to_string(),
            Err(_) => path.clone(),
        }
    }
}

pub fn get_month_number(file: &File) -> Result<u32, io::Error> {
    // Get the metadata of the file
    let metadata = file.metadata()?;

    // Extract the modification time from the metadata
    let modified_time = metadata.modified()?;

    // Convert the modification time (seconds since UNIX_EPOCH) into a SystemTime
    let system_time = SystemTime::UNIX_EPOCH + modified_time.duration_since(UNIX_EPOCH).unwrap();

    // Extract the month from the broken-down time
    let month = time_manipulator::get_month_number_from_systemtime(system_time);
    println!("{:#?}", month);

    Ok(month)
}

pub fn get_year_number(file: &File) -> Result<i32, io::Error> {
    // Get the metadata of the file
    let metadata = file.metadata()?;

    // Extract the modification time from the metadata
    let modified_time = metadata.modified()?;

    // Convert the modification time (seconds since UNIX_EPOCH) into a SystemTime
    let system_time = SystemTime::UNIX_EPOCH + modified_time.duration_since(UNIX_EPOCH).unwrap();

    let year = time_manipulator::get_year_number_from_systemtime(system_time);

    Ok(year)
}

pub fn move_file(from: PathBuf, to: PathBuf, create_dir_if_missing: bool) -> io::Result<()> {
    if create_dir_if_missing {
        let to_parent_dir = match to.parent() {
            Some(value) => value,
            None => Path::new("/"),
        };

        match fs::create_dir_all(to_parent_dir) {
            Ok(_) => (),
            Err(io_error) => return Err(io_error),
        };
    }

    fs::rename(from, to)
}
