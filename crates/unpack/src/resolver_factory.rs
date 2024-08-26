use std::{hash::{DefaultHasher, Hash}, sync::Arc};

use dashmap::DashMap;
use rspack_resolver::ResolveOptions;

use crate::resolver::UnpackResolver;
#[derive(Debug)]
pub struct ResolverFactory {
    base_options: ResolveOptions,
    pub base_resolver: UnpackResolver,
    // resolver_cache: DashMap<ResolveOptions, Arc<UnpackResolver>>
}

impl ResolverFactory {
    pub fn new()-> Self{
        Self {
            base_options: ResolveOptions::default(),
            base_resolver: UnpackResolver::new(ResolveOptions::default()),
        }
    }
    pub fn new_with_base_option(options: ResolveOptions)-> Self{
        Self {
            base_options: options.clone(),
            base_resolver: UnpackResolver::new(options.clone()),
        }
    }
    pub fn get(&self,options: ResolveOptions) -> Arc<UnpackResolver>{
         // FIXME: support resolver cache
         Arc::new(self.base_resolver.clone_with_options(options))
    }
}