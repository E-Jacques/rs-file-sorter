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
