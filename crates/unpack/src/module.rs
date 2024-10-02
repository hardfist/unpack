mod module_id;
use std::fmt::Debug;

pub use module_id::*;
use rspack_sources::BoxSource;

use crate::errors::Diagnostics;

pub trait Module: Debug {}

pub type BoxModule = Box<dyn Module>;
#[derive(Debug)]
pub struct ModuleIdentifier(String);

// #[derive(Debug)]
// pub struct NormalModuleDraft {
//     diagnostics: Diagnostics,
//     original_source: Option<BoxSource>,
// }
#[derive(Debug)]
pub struct NormalModule {
    id: ModuleIdentifier,
    diagnostics: Diagnostics,
    original_source: Option<BoxSource>,
}
impl Module for NormalModule {
    
}
impl NormalModule {
    pub(crate) fn new(request: String) -> Self {
        let id = Self::gen_id(&request);
        Self {
            id,
            diagnostics: vec![],
            original_source: None,
        }
    }
    fn gen_id(request: &str) -> ModuleIdentifier {
        ModuleIdentifier(request.to_string())
    }
}
