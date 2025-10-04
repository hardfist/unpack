pub use miette;
use miette::Diagnostic;
use miette::Report;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("resolve error")]
#[diagnostic()]
pub struct ResolveError {
    #[source]
    source: rspack_resolver::ResolveError,
}

pub type Diagnostics = Vec<Report>;
