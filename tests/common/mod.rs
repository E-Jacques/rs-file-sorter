pub mod file_creator;
use filetime::{set_file_times, FileTime};
use std::{fs::{self, File}, io, path::PathBuf};

use self::file_creator::FileCreator;

const ISO_DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub fn file_or_dir_exists(path: PathBuf) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn clean_dir(path: PathBuf) -> io::Result<()> {
    match fs::remove_dir_all(path.clone()) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    fs::create_dir(path.clone())
}

fn iso_date_to_filetime(iso_date: &str) -> Result<FileTime, chrono::ParseError> {
    // Parse the ISO date string to a NaiveDateTime
    let datetime = chrono::NaiveDateTime::parse_from_str(iso_date, ISO_DATE_FORMAT)?;

    // Convert the NaiveDateTime to a FileTime
    let filetime = FileTime::from_unix_time(datetime.timestamp(), 0);

    Ok(filetime)
}

pub fn set_file_access_and_modification_time(
    path: PathBuf,
    access_time: &str,
    modification_time: &str,
) -> io::Result<()> {
    let access_file_time = iso_date_to_filetime(access_time)
        .expect("ISO date should respect the following format: %Y-%m-%dT%H:%M:%S%.f");
    let modification_file_time = iso_date_to_filetime(modification_time)
        .expect("ISO date should respect the following format: %Y-%m-%dT%H:%M:%S%.f");

    set_file_times(path, access_file_time, modification_file_time)
}

pub fn to_iso_date(year: u16, month: u8, day: u8) -> String {
    format!("{year}-{month}-{day}T00:00:00.000")
}

pub fn generate_test_files(target_dir: &PathBuf, files: Vec<FileCreator>) -> std::io::Result<()> {

    for filename in files {
        let filepath = target_dir.clone().join(filename.path.clone());

        match File::create(filepath.clone()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        
        let last_modified_time = &to_iso_date(filename.year, filename.month, filename.day);
        match set_file_access_and_modification_time(
            filepath.clone(),
            last_modified_time,
            last_modified_time,
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}