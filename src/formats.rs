use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormats {
    Txt,
    Csv,
    Json,
    Md
}

impl FileFormats{
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();

        match ext.as_str() {
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "txt" => Some(Self::Txt),
            "md" => Some(Self::Md),
            _ => None,
        }
    }
}