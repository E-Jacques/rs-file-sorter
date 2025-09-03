pub fn file_ext(file_path: &std::path::PathBuf) -> String {
    file_path
        .extension()
        .map(|os_str| os_str.to_str())
        .flatten()
        .unwrap_or("unknown")
        .to_string()
}
