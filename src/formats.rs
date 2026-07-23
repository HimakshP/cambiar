use std::{fmt, path::Path};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FileFormats {
    Txt,
    Csv,
    Json,
    Md,
    Png,
    Jpg
}

impl FileFormats {
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();

        match ext.as_str() {
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "txt" => Some(Self::Txt),
            "md" => Some(Self::Md),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpg),
            _ => None,
        }
    }
}

impl fmt::Display for FileFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileFormats::Csv => write!(f, "CSV"),
            FileFormats::Json => write!(f, "JSON"),
            FileFormats::Txt => write!(f, "Text"),
            FileFormats::Md => write!(f, "Markdown"),
            FileFormats::Png => write!(f, "Png"),
            FileFormats::Jpg => write!(f, "Jpg"),
        }
    }
}
