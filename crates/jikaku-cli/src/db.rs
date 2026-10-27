use std::{fs, path::Path};

use anyhow::Result;
use clap::Args;

use crate::kanjivg::{Character, parse_kvg};

pub const DEFAULT_CURVE_STEPS: u16 = 20;
pub const DEFAULT_GRID_SIZE: u16 = 256;
pub const DEFAULT_POINT_DISTANCE: u16 = 20;

#[derive(Args, Debug)]
pub struct BuildDbArgs {
    dir_path: String,
    #[arg(long, short = 'c', default_value_t = DEFAULT_CURVE_STEPS)]
    curve_steps: u16,
    #[arg(long, short = 's', default_value_t = DEFAULT_GRID_SIZE)]
    grid_size: u16,
    #[arg(long, short = 'd', default_value_t = DEFAULT_POINT_DISTANCE)]
    point_distance: u16,
    #[arg(long, short, default_value_t = "characters.bin".to_string())]
    output_path: String,
}

pub fn build_db(args: BuildDbArgs) -> Result<()> {
    let path_buf = Path::new(&args.dir_path);
    let entries = path_buf.read_dir()?;
    let mut characters = Vec::new();

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        if file_name.to_string_lossy().ends_with(".svg") {
            let content = fs::read_to_string(entry.path())?;
            let base_char = parse_kvg(&content, args.curve_steps)?;
            let normalized_char = base_char.normalize(args.grid_size);
            let simplified_char = normalized_char.simplify(args.point_distance);

            characters.push(simplified_char);
        }
    }

    let content = create_binary(&characters)?;

    fs::write(args.output_path, content)?;

    Ok(())
}

fn create_binary(characters: &[Character<u8>]) -> Result<String> {
    todo!()
}
