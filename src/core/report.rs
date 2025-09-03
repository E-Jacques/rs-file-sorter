pub type FullReport = Vec<Report>;

#[derive(Clone, Debug)]
pub struct Report {
    pub input_filename: std::path::PathBuf,
    pub result: Result<std::path::PathBuf, std::rc::Rc<super::error::Error>>,
}
