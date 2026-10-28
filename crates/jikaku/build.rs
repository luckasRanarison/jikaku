use std::{env, error::Error, path::PathBuf};

use jikaku_builder::{BuildOptions, build_db};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let kanji_dir = manifest_dir.join("../../submodules/kanjivg/kanji");
    let output_path = manifest_dir.join("./bin/data.bin");

    if !output_path.exists() {
        build_db(kanji_dir, output_path, &BuildOptions::default())?;
    }

    Ok(())
}
