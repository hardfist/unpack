use camino::{Utf8Path, Utf8PathBuf};
use rspack_resolver::{ResolveOptions, Resolver, ResolveError as InnerResolverError};

use crate::utils::path::AssertUtf8;
pub struct NormalResolver {
    inner_resolver:Resolver 
}

pub struct ResolveResult {
    pub path: Utf8PathBuf
}
pub type ResolveError = InnerResolverError;
impl NormalResolver {
    pub fn new(options: ResolveOptions) -> Self{
        Self {
            inner_resolver: Resolver::new(options)
        }
    }
    pub fn resolve(&self,context: &Utf8Path, request: &str) -> Result<ResolveResult, ResolveError>{
        self.inner_resolver.resolve(context, request).map(|resolution| {
            let full_path = resolution.full_path();
            ResolveResult {
                path: full_path.assert_utf8()
            }
        })
    }
}
