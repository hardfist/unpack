use std::fs::File;

#[derive(Clone,Default)]
pub struct Files {

}
impl Files {
    pub fn new() -> Self {
        Files {
            
        }
    }
    pub fn open(&self, path: &str) -> std::io::Result<File> {
        File::open(path)
    }
}