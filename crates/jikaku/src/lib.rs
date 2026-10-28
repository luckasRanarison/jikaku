use jikaku_bin::{reader::BinaryReader, types::Point};
use jikaku_builder::processing::{euclidian_reduction, linear_normalization};
pub use jikaku_builder::types::{RawPoint, Stroke};

const CANDIDATE_COUNT: usize = 20;

// FIXME: should be allocation free
pub fn match_kanji(strokes: &[Stroke<RawPoint>]) -> [u32; CANDIDATE_COUNT] {
    static DATA: &[u8] = include_bytes!("../bin/data.bin");

    let reader = BinaryReader::new(DATA);
    let normalized = linear_normalization(strokes, reader.get_grid_size());
    let reduced = euclidian_reduction(&normalized, reader.get_min_point_distance());
    let stroke_count = reduced.len();

    let mut candidates = [u32::MAX; CANDIDATE_COUNT];
    let mut distances = [f64::INFINITY; CANDIDATE_COUNT];

    let Some(characters) = reader.get_chars_by_stroke(stroke_count) else {
        return candidates;
    };

    for character in characters {
        let mut distance = 0.0;

        for (input_stroke, stroke) in reduced.iter().zip(reader.get_strokes(&character)) {
            distance += initial_stroke_distance(
                input_stroke.points.iter().copied(),
                reader.get_points(&stroke),
            );
        }

        insert_candidate(&mut candidates, &mut distances, character.id, distance);
    }

    candidates
}

fn initial_stroke_distance<I, J>(first: I, second: J) -> f64
where
    I: ExactSizeIterator<Item = Point>,
    J: ExactSizeIterator<Item = Point>,
{
    if first.len() >= second.len() {
        _initial_stroke_distance(first, second)
    } else {
        _initial_stroke_distance(second, first)
    }
}

fn _initial_stroke_distance<I, J>(longest: I, shortest: J) -> f64
where
    I: ExactSizeIterator<Item = Point>,
    J: ExactSizeIterator<Item = Point>,
{
    let n = longest.len();
    let m = shortest.len();

    if m == 0 {
        return f64::INFINITY;
    }

    let mut shortest = shortest.enumerate();
    let mut current = shortest.next();
    let mut total = 0;

    for (i, point) in longest.enumerate() {
        // map points of the shortest one to the longest
        let target = if n > 1 { i * (m - 1) / (n - 1) } else { 0 };

        while current.is_some_and(|(index, _)| index < target) {
            current = shortest.next();
        }

        let Some((_, other)) = current else {
            return f64::INFINITY;
        };

        total += point.x.abs_diff(other.x) as u32 + point.y.abs_diff(other.y) as u32;
    }

    (n as f64 / m as f64) * total as f64
}

fn insert_candidate(
    candidates: &mut [u32; CANDIDATE_COUNT],
    distances: &mut [f64; CANDIDATE_COUNT],
    id: u32,
    distance: f64,
) {
    if distance.is_infinite() {
        return;
    }

    let Some(position) = distances.iter().position(|&value| distance < value) else {
        return;
    };

    for index in (position + 1..CANDIDATE_COUNT).rev() {
        candidates[index] = candidates[index - 1];
        distances[index] = distances[index - 1];
    }

    candidates[position] = id;
    distances[position] = distance;
}
