#![deny(clippy::all)]
use camino::Utf8PathBuf;
use napi::bindgen_prelude::{block_on, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Either;
use unpack::compiler::EntryItem;
use unpack::resolver::ResolveOptions;
use unpack::{bundler::unpack, compiler::CompilerOptions};
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn build(
    context: String,
    entry: String,
    callback: ThreadsafeFunction<u32>,
) -> napi::Result<()> {
    let (send, recv) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        callback.call_with_return_value(
            Ok(32),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            move |ret: Either<Promise<String>, String>| {
                send.send(ret).unwrap();
                Ok(())
            },
        );
    });
    std::thread::spawn(move || {
        let call_result = match recv.recv().unwrap() {
            Either::A(p) => block_on(p).unwrap(),
            Either::B(b) => b,
        };
        unpack(CompilerOptions {
            context: Utf8PathBuf::from(context),
            entry: vec![EntryItem {
                name: "main".to_string(),
                import: entry,
            }],
            resolve: ResolveOptions {
                extensions: vec![".js", ".ts", ".mjs"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
                ..Default::default()
            },
        });
    });

    Ok(())
}
