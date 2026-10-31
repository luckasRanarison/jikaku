mod utils;

pub use jikaku_builder::types::{RawPoint, Stroke};

use jikaku_bin::{
    reader::BinaryReader,
    types::{CharacterRef, Point},
};
use jikaku_builder::processing::{euclidian_reduction, linear_normalization};

static DATA: &[u8] = include_bytes!("../bin/data.bin");

static READER: BinaryReader = BinaryReader::new(DATA);

const GRID_SIZE: u8 = READER.get_grid_size();
const MIN_POINT_DISTANCE: u8 = READER.get_min_point_distance();

const CANDIDATE_COUNT: usize = 20;
const STROKE_THRESHOLD: usize = 3;
const MAP_SIZE: usize = 35;

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

type CandidateList = [Candidate; CANDIDATE_COUNT];

pub fn match_character(strokes: &[Stroke<RawPoint>]) -> CandidateList {
    let normalized = linear_normalization(strokes, GRID_SIZE);
    let input_strokes = euclidian_reduction(&normalized, MIN_POINT_DISTANCE);
    let stroke_count = input_strokes.len();

    let mut candidates = [Candidate::default(); CANDIDATE_COUNT];

    let Some(characters) = READER.get_chars_by_stroke(stroke_count) else {
        return candidates;
    };

    for character in characters {
        test_character(&character, &input_strokes, &mut candidates);
    }

    candidates
}

fn test_character(
    character: &CharacterRef,
    input_strokes: &[Stroke<Point>],
    candidates: &mut CandidateList,
) {
    let mut distance = 0.0;

    for (input, strokes) in input_strokes.iter().zip(character.strokes()) {
        distance += stroke_distance(input.points.iter().copied(), strokes.points());
    }

    insert_candidate(candidates, character.id, distance);
}

// fn init_stroke_map() {}

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

fn insert_candidate(candidates: &mut CandidateList, id: u32, distance: f64) {
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
