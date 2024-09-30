use crate::compiler::{Compiler, CompilerOptions};
pub fn unpack(options: CompilerOptions) {
    let mut compiler = Compiler::new(options);
    compiler.build();
}
