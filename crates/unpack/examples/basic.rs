use rspack_resolver::ResolveOptions;
use std::{path::PathBuf, sync::Arc};
use unpack::{
    compiler::{Compiler, CompilerOptions, EntryItem},
    tracing::init_tracing,
};

#[tokio::main]
async fn main() {
    let guard = init_tracing();

    fn create_compiler(dist: String) -> Compiler {
        let root =  env!("CARGO_MANIFEST_DIR");
        let context = PathBuf::from(root)
            .join("./examples/fixtures");
        let context = context.canonicalize().expect("expect canonicalize success");
        let compiler_options: CompilerOptions = CompilerOptions {
            output_dir: context.join(dist).try_into().expect("expect utf8 path"),
            context: context.try_into().expect("expect utf8 path"),
            entry: vec![
                EntryItem {
                    name: "main".to_string(),
                    import: "./src/index.mjs".to_string(),
                },
                EntryItem {
                    name: "other".to_string(),
                    import: "./src/other.mjs".to_string(),
                },
            ],
            resolve: ResolveOptions {
                extensions: vec![".js", ".ts", ".mjs"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
                ..Default::default()
            },
        };
        tracing::debug!("compiler_options: {:?}", compiler_options);
        let compiler = Compiler::new(Arc::new(compiler_options), vec![]);
        compiler
    }
    let mut compiler_1 = create_compiler("dist1".to_string());
    let mut compiler_2 = create_compiler("dist2".to_string());
    let p1 = compiler_1.build();
    let p2 = compiler_2.build();
    let _ = tokio::join!(p1, p2);
    drop(guard);
}
