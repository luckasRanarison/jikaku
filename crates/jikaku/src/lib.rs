pub use jikaku_builder::types::{RawPoint, Stroke};

use jikaku_bin::{reader::BinaryReader, types::Point};
use jikaku_builder::processing::{euclidian_reduction, linear_normalization};

const CANDIDATE_COUNT: usize = 20;

#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    pub id: u32,
    pub distance: f64,
}

impl Default for Candidate {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            distance: f64::INFINITY,
        }
    }
}

pub fn match_character(strokes: &[Stroke<RawPoint>]) -> [Candidate; CANDIDATE_COUNT] {
    static DATA: &[u8] = include_bytes!("../bin/data.bin");

    let reader = BinaryReader::new(DATA);
    let normalized = linear_normalization(strokes, reader.get_grid_size());
    let reduced = euclidian_reduction(&normalized, reader.get_min_point_distance());
    let stroke_count = reduced.len();

    let mut candidates = [Candidate::default(); CANDIDATE_COUNT];

    let Some(characters) = reader.get_chars_by_stroke(stroke_count) else {
        return candidates;
    };

    for character in characters {
        let mut distance = 0.0;

        for (input_stroke, stroke) in reduced.iter().zip(reader.get_strokes(&character)) {
            distance += stroke_distance(
                input_stroke.points.iter().copied(),
                reader.get_points(&stroke),
            );
        }

        insert_candidate(&mut candidates, character.id, distance);
    }

    candidates
}

fn stroke_distance<I, J>(first: I, second: J) -> f64
where
    I: ExactSizeIterator<Item = Point>,
    J: ExactSizeIterator<Item = Point>,
{
    if first.len() >= second.len() {
        _stroke_distance(first, second)
    } else {
        _stroke_distance(second, first)
    }
}

// whole-stroke distance
fn _stroke_distance<I, J>(longest: I, shortest: J) -> f64
where
    I: ExactSizeIterator<Item = Point>,
    J: ExactSizeIterator<Item = Point>,
{
    let n = longest.len();
    let m = shortest.len();

    if m == 0 {
        return f64::INFINITY;
    }

    let mut distance = 0;
    let mut longest_iter = longest.enumerate();

    for (i, s1) in shortest.enumerate() {
        let j = match m {
            1 => 0,
            _ => (n - 1) / (m - 1) * i,
        };

        for (k, s2) in longest_iter.by_ref() {
            if j >= k {
                distance += s1.x.abs_diff(s2.x) as u32 + s1.y.abs_diff(s2.y) as u32;
                break;
            }
        }
    }

    (n as f64 * distance as f64) / m as f64
}

fn insert_candidate(candidates: &mut [Candidate; CANDIDATE_COUNT], id: u32, distance: f64) {
    if distance.is_infinite() {
        return;
    }

    let Some(position) = candidates.iter().position(|c| distance < c.distance) else {
        return;
    };

    for index in (position + 1..CANDIDATE_COUNT).rev() {
        candidates[index] = candidates[index - 1];
    }

    candidates[position].id = id;
    candidates[position].distance = distance;
}
