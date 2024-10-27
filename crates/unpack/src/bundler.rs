use std::sync::Arc;

use crate::{
    compiler::{Compiler, CompilerOptions},
    plugin::BoxPlugin,
};
pub fn unpack(options: CompilerOptions, plugins: Vec<BoxPlugin>) {
    let mut compiler = Compiler::new(Arc::new(options), plugins);
    compiler.build();
}
