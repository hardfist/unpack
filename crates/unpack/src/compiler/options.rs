
use camino::Utf8PathBuf;
#[derive(Clone)]
pub struct EntryItem {
    pub name: String,
    pub import: String,
}

#[derive(Clone)]
pub struct CompilerOptions {
    pub context: Utf8PathBuf,
    pub entry: EntryItem,
}
