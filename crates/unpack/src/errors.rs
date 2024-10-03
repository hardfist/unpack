pub use miette;
use miette::Diagnostic;
use thiserror::Error;
use miette::Report;

#[derive(Debug, Error, Diagnostic)]
#[error("resolve error")]
#[diagnostic()]
pub struct ResolveError {
    #[source]
    source: rspack_resolver::ResolveError,
}

pub type Diagnostics = Vec<Report>;

