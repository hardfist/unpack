use std::path::PathBuf;

#[salsa::input]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String
}
