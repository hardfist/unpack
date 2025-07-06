
use clap::Parser;
use unpack_ng_cli::{arguments::Arguments, build, register};
fn main() -> anyhow::Result<()> {
    register();
    let arguments =  Arguments::parse();
    dbg!(arguments);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(build())?;
    Ok(())
}

