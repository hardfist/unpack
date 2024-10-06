use camino::Utf8PathBuf;
use rspack_resolver::ResolveOptions;
#[derive(Clone, Debug)]
pub struct EntryItem {
    pub name: String,
    pub import: String,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub context: Utf8PathBuf,
    pub entry: Vec<EntryItem>,
    pub resolve: ResolveOptions,
}
