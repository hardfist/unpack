use std::{env, path::PathBuf};

use unpack_ng::{compiler::dev, db};


fn main() -> anyhow::Result<()> {
    dbg!(env!("CARGO_MANIFEST_DIR"));
    let fixture_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./examples/fixtures").canonicalize().unwrap();
    let entry = std::env::args().nth(1).unwrap_or_else( || {
        fixture_root.join("simple/index.js").canonicalize().unwrap().to_string_lossy().to_string()
    });
    dev(PathBuf::from(entry))?;
    Ok(())
}
