use camino::Utf8PathBuf;
use std::path::PathBuf;
pub trait AssertUtf8 {
    type Output;
    fn assert_utf8(self) -> Self::Output;
}
impl AssertUtf8 for PathBuf {
    type Output = Utf8PathBuf;

    /// Assert `self` is a valid UTF-8 [`PathBuf`] and convert to [`Utf8PathBuf`]
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a valid UTF-8 path.
    fn assert_utf8(self) -> Self::Output {
        Utf8PathBuf::from_path_buf(self).unwrap_or_else(|p| {
            panic!("expected UTF-8 path, got: {}", p.display());
        })
    }
}
