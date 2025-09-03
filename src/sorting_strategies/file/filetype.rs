#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileType {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Executable,
    #[default]
    Other,
}

impl Into<String> for FileType {
    fn into(self) -> String {
        match self {
            FileType::Image => "image".to_string(),
            FileType::Video => "video".to_string(),
            FileType::Audio => "audio".to_string(),
            FileType::Document => "document".to_string(),
            FileType::Archive => "archive".to_string(),
            FileType::Executable => "executable".to_string(),
            FileType::Other => "other".to_string(),
        }
    }
}

impl From<String> for FileType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "image" => FileType::Image,
            "video" => FileType::Video,
            "audio" => FileType::Audio,
            "document" => FileType::Document,
            "archive" => FileType::Archive,
            "executable" => FileType::Executable,
            _ => FileType::Other,
        }
    }
}

impl FileType {
    pub fn from_extension(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "bmp" => FileType::Image,
            "mp4" | "avi" | "mkv" | "mov" => FileType::Video,
            "mp3" | "wav" | "flac" => FileType::Audio,
            "pdf" | "docx" | "txt" => FileType::Document,
            "zip" | "rar" | "tar.gz" => FileType::Archive,
            _ if extension.is_empty() => FileType::Other,
            _ => FileType::Executable,
        }
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
