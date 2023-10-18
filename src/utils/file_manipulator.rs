use std::path::{Path, PathBuf};
use std::{fs, io};

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
