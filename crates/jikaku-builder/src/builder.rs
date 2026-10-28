use std::{fs, path::Path};

use anyhow::Result;
use jikaku_bin::{encoding::*, types::Point};

use crate::{
    kanjivg::parse_kvg,
    processing::{euclidian_reduction, linear_normalization},
    types::Character,
};

pub const DEFAULT_CURVE_STEPS: u8 = 20;
pub const DEFAULT_GRID_SIZE: u16 = 256;
pub const DEFAULT_POINT_DISTANCE: u8 = 20;

#[derive(Debug, Clone, Copy)]
pub struct BuildOptions {
    pub grid_size: u16,
    pub curve_steps: u8,
    pub point_distance: u8,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            curve_steps: DEFAULT_CURVE_STEPS,
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

        let file_name = file_name.to_string_lossy();

        // skip character variants
        if file_name.ends_with(".svg") && !file_name.contains('-') {
            let content = fs::read_to_string(entry.path())?;
            let base_char = parse_kvg(&content, options.curve_steps)?;
            let normalized_strokes = linear_normalization(&base_char.strokes, options.grid_size);
            let simplified_strokes =
                euclidian_reduction(&normalized_strokes, options.point_distance);

            characters.push(Character {
                id: base_char.id,
                strokes: simplified_strokes,
            });
        }
    }

    characters.sort_by_key(|c| (c.strokes.len(), c.id));

    let content = create_binary(&characters, options)?;
    let output_path = output_path.as_ref();

    fs::write(output_path, content)?;

    Ok(())
}

fn create_binary(characters: &[Character<Point>], options: &BuildOptions) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    write_header(&mut buffer, characters, options)?;

    let max_stroke = characters.last().map_or(0, |c| c.strokes.len());
    let total_strokes: usize = characters.iter().map(|c| c.strokes.len()).sum();
    let mut start = 0;

    for stroke_count in 1..=max_stroke {
        let mut end = start;

        while end < characters.len() && characters[end].strokes.len() == stroke_count {
            end += 1;
        }

        buffer.extend((start as u16).to_le_bytes());
        buffer.extend(((end - start) as u16).to_le_bytes());
        start = end;
    }

    let header_and_map_size = HEADER_SIZE + (max_stroke * CHAR_MAP_ENTRY_SIZE);
    let char_entries_size = characters.len() * CHAR_ENTRY_SIZE;
    let stroke_entries_size = total_strokes * STROKE_ENTRY_SIZE;

    let stroke_entries_base = header_and_map_size + char_entries_size;
    let points_base = stroke_entries_base + stroke_entries_size;

    let mut char_entries_buf = Vec::with_capacity(char_entries_size);
    let mut stroke_entries_buf = Vec::with_capacity(stroke_entries_size);
    let mut points_buf = Vec::new();

    let mut current_stroke_entry_offset = stroke_entries_base as u32;
    let mut current_point_offset = points_base as u32;

    for character in characters {
        let stroke_count = character.strokes.len() as u8;
        let stroke_offset = current_stroke_entry_offset;

        char_entries_buf.extend(character.id.to_le_bytes());
        char_entries_buf.push(stroke_count);
        char_entries_buf.extend(stroke_offset.to_le_bytes());
        current_stroke_entry_offset += (stroke_count as usize * STROKE_ENTRY_SIZE) as u32;

        for stroke in &character.strokes {
            let point_count = u8::try_from(stroke.points.len())?;
            let point_offset = current_point_offset;

            stroke_entries_buf.extend(point_count.to_le_bytes());
            stroke_entries_buf.extend(point_offset.to_le_bytes());
            current_point_offset += (stroke.points.len() * POINT_ENTRY_SIZE) as u32;

            for point in &stroke.points {
                points_buf.push(point.x);
                points_buf.push(point.y);
            }
        }
    }

    buffer.extend(char_entries_buf);
    buffer.extend(stroke_entries_buf);
    buffer.extend(points_buf);

    Ok(buffer)
}

fn write_header(
    buffer: &mut Vec<u8>,
    characters: &[Character<Point>],
    options: &BuildOptions,
) -> Result<()> {
    let max_stroke_count = characters
        .last()
        .map(|c| c.strokes.len())
        .unwrap_or_default();

    buffer.extend(MAGIC);
    buffer.extend(u16::try_from(characters.len())?.to_le_bytes());
    buffer.push(u8::try_from(max_stroke_count)?);
    buffer.extend(options.grid_size.to_le_bytes());
    buffer.push(options.curve_steps);
    buffer.extend(options.point_distance.to_le_bytes());

    Ok(())
}
