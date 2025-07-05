use unpack_ng::{main_inner, register};
fn main() -> anyhow::Result<()> {
    register();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(main_inner())?;
    Ok(())
}

