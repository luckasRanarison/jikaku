use std::{fmt::Write, fs, path::Path};

use anyhow::Result;

use crate::kanjivg::{Character, parse_kvg};

pub const DEFAULT_CURVE_STEPS: u16 = 20;
pub const DEFAULT_GRID_SIZE: u16 = 256;
pub const DEFAULT_POINT_DISTANCE: u16 = 20;

#[derive(Debug, Clone, Copy)]
pub struct BuildOptions {
    pub curve_steps: u16,
    pub grid_size: u16,
    pub point_distance: u16,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            curve_steps: DEFAULT_CURVE_STEPS,
            grid_size: DEFAULT_GRID_SIZE,
            point_distance: DEFAULT_POINT_DISTANCE,
        }
    }
}

pub fn build_db(
    dir_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    options: &BuildOptions,
) -> Result<()> {
    let entries = fs::read_dir(dir_path)?;
    let mut characters = Vec::new();

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        if file_name.to_string_lossy().ends_with(".svg") {
            let content = fs::read_to_string(entry.path())?;
            let base_char = parse_kvg(&content, options.curve_steps)?;
            let normalized_char = base_char.normalize(options.grid_size);
            let simplified_char = normalized_char.simplify(options.point_distance);

            characters.push(simplified_char);
        }
    }

    if output_path.as_ref().exists() {
        fs::remove_file(&output_path)?;
    }

    let content = create_rust_file(&characters)?;
    let output_path = output_path.as_ref();

    fs::write(output_path, content)?;

    Ok(())
}

fn create_rust_file(characters: &[Character<u8>]) -> Result<String> {
    let mut code = String::new();

    writeln!(
        code,
        "#[derive(Debug, Clone, Copy)]
pub struct Point {{
    pub x: u8,
    pub y: u8,
}}

#[derive(Debug, Clone, Copy)]
pub struct Stroke {{
    pub points: &'static [Point],
}}

#[derive(Debug, Clone, Copy)]
pub struct Character {{
    pub id: u32,
    pub strokes: &'static [Stroke],
}}

pub static CHARACTER_DB: &[Character] = &["
    )?;

    for character in characters {
        writeln!(
            code,
            "    Character {{\n        id: {},\n        strokes: &[",
            character.id
        )?;

        for stroke in &character.strokes {
            writeln!(code, "            Stroke {{")?;
            writeln!(code, "                points: &[")?;

            for point in &stroke.points {
                writeln!(
                    code,
                    "                    Point {{ x: {}, y: {} }},",
                    point.x, point.y
                )?;
            }

            writeln!(code, "                ],")?;
            writeln!(code, "            }},")?;
        }

        writeln!(code, "        ],\n    }},")?;
    }

    writeln!(code, "];")?;

    Ok(code)
}
