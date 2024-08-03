use std::sync::Arc;
use derive_new::new;
use crate::CompilerOptions;

#[derive(new)]
pub struct Compilation {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>
}

impl Compilation {
    /// similar with webpack's make phase, which will make module graph
    pub fn scan(&mut self){
        println!("start scan")
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self){
        println!("start link")
    }

}