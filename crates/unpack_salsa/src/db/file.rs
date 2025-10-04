use std::path::PathBuf;

/// The raw source of module
#[salsa::input]
pub struct FileSource {
    /// absolute path of the file
    pub path: PathBuf,
    pub content: String
}
