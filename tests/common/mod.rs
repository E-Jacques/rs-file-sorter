use std::{fs, path::PathBuf, io};

pub fn file_or_dir_exists (path: PathBuf) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn clean_dir (path: PathBuf) -> io::Result<()> {
    match fs::remove_dir_all(path.clone()) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };
    
    fs::create_dir(path.clone())
}