use thiserror::Error;
use miette::{Diagnostic};

#[derive(Debug, Error, Diagnostic)]
#[error("resolve error")]
#[diagnostic()]
pub struct ResolveError {
    #[source]
    source: rspack_resolver::ResolveError
}

pub type UnpackDiagnostic = Box<dyn Diagnostic + Send + Sync + 'static>;