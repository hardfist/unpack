use std::sync::Arc;

use rspack_resolver::ResolveOptions;

use crate::resolver::UnpackResolver;
#[derive(Debug)]
pub struct ResolverFactory {
    base_options: ResolveOptions,
    pub base_resolver: UnpackResolver,
}

impl ResolverFactory {
    pub fn new_with_base_option(options: ResolveOptions) -> Self {
        Self {
            base_options: options.clone(),
            base_resolver: UnpackResolver::new(options.clone()),
        }
    }
    pub fn get(&self, options: ResolveOptions) -> Arc<UnpackResolver> {
        // FIXME: support resolver cache
        Arc::new(self.base_resolver.clone_with_options(options))
    }
}
