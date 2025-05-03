use std::{fmt::Debug, hash::BuildHasherDefault};

use dyn_clone::DynClone;
use indexmap::IndexMap;
use miette::Result;
use rspack_sources::{BoxSource, ConcatSource, RawStringSource, SourceExt};
use rustc_hash::FxHasher;

use crate::utils::ext::{DynHash, IntoAny};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InitFragmentStage {
    StageConstants,
    StageAsyncBoundary,
    StageESMExports,
    StageESMImports,
    StageProvides,
    StageAsyncDependencies,
    StageAsyncESMImports,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum InitFragmentKey {
    Unique(u32),
    ESMImport(String),
    ESMExportStar(String), // TODO: align with webpack and remove this
    ESMExports,
    CommonJsExports(String),
    ModuleExternal(String),
    ExternalModule(String),
    AwaitDependencies,
    ESMCompatibility,
    ModuleDecorator(String /* module_id */),
    ESMFakeNamespaceObjectFragment(String),
    Const(String),
}
pub struct InitFragmentContents {
    pub start: String,
    pub end: Option<String>,
}
impl InitFragmentKey {
    pub fn merge_fragments<C>(
        &self,
        fragments: Vec<Box<dyn InitFragment<C>>>,
    ) -> Box<dyn InitFragment<C>> {
        fragments
            .into_iter()
            .next()
            .expect("shoud a least have one fragment")
    }
}
pub trait InitFragment<C>: IntoAny + DynHash + DynClone + Debug + Sync + Send {
    fn stage(&self) -> InitFragmentStage;
    fn position(&self) -> i32;
    fn key(&self) -> &InitFragmentKey;
    fn contents(self: Box<Self>, context: &mut C) -> Result<InitFragmentContents>;
}
pub trait InitFragmentRenderContext {}

pub fn render_init_fragments<C: InitFragmentRenderContext>(
    source: BoxSource,
    mut fragments: Vec<Box<dyn InitFragment<C>>>,
    context: &mut C,
) -> Result<BoxSource> {
    fragments.sort_by(|a, b| {
        let stage = a.stage().cmp(&b.stage());
        if !stage.is_eq() {
            return stage;
        }
        a.position().cmp(&b.position())
    });
    let mut keyed_fragments: IndexMap<
        InitFragmentKey,
        Vec<Box<dyn InitFragment<C>>>,
        BuildHasherDefault<FxHasher>,
    > = IndexMap::default();
    for fragment in fragments {
        let key = fragment.key();
        if let Some(value) = keyed_fragments.get_mut(key) {
            value.push(fragment);
        } else {
            keyed_fragments.insert(key.clone(), vec![fragment]);
        }
    }

    let mut end_contents = vec![];
    let mut concat_source = ConcatSource::default();
    for (key, fragments) in keyed_fragments {
        let f = key.merge_fragments(fragments);
        let contents = f.contents(context)?;
        concat_source.add(RawStringSource::from(contents.start));
        if let Some(end_content) = contents.end {
            end_contents.push(RawStringSource::from(end_content));
        }
    }
    concat_source.add(source);
    for content in end_contents.into_iter().rev() {
        concat_source.add(content);
    }

    Ok(concat_source.boxed())
}
