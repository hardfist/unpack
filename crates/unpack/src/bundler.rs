use crate::compiler::{CompilerOptions,Compiler};
pub fn unpack(options: CompilerOptions){
    let mut compiler = Compiler::new(options);
    compiler.build();
}