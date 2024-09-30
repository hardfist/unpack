use camino::{Utf8Path, Utf8PathBuf};
use rspack_resolver::{ResolveError as InnerResolverError, ResolveOptions, Resolver};

use crate::utils::path::AssertUtf8;

#[derive(Debug)]
pub struct UnpackResolver {
    inner_resolver: Resolver,
}

#[derive(Debug)]
pub struct ResolveResult {
    pub path: Utf8PathBuf,
}
pub type ResolveError = InnerResolverError;
impl UnpackResolver {
    pub fn new(options: ResolveOptions) -> Self {
        Self {
            inner_resolver: Resolver::new(options),
        }
    }
    pub fn clone_with_options(&self, options: ResolveOptions) -> Self {
        let resolver = self.inner_resolver.clone_with_options(options);
        Self {
            inner_resolver: resolver,
        }
    }
    pub fn resolve(
        &self,
        context: &Utf8Path,
        request: &str,
    ) -> Result<ResolveResult, ResolveError> {
        self.inner_resolver
            .resolve(context, request)
            .map(|resolution| {
                let full_path = resolution.full_path();
                ResolveResult {
                    path: full_path.assert_utf8(),
                }
            })
    }
}
