use std::path::PathBuf;

pub type FullSorterReport = Vec<SorterReport>;

#[derive(Clone, Debug)]
pub struct SorterReport {
    pub input_filename: PathBuf,
    pub result: Result<PathBuf, std::rc::Rc<super::error::Error>>,
}

#[derive(Clone, Debug)]
pub struct SortOptions {
    pub dry_run: bool,
    pub root_level_only: bool,
}

impl Default for SortOptions {
    fn default() -> Self {
        SortOptions {
            dry_run: false,
            root_level_only: false,
        }
    }
}
