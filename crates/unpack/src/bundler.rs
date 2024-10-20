use std::sync::Arc;

use crate::{
    compiler::{Compiler, CompilerOptions},
    plugin::Plugin,
};
pub fn unpack(options: CompilerOptions, plugins: Vec<Box<dyn Plugin>>) {
    let mut compiler = Compiler::new(Arc::new(options), plugins);
    compiler.build();
}
