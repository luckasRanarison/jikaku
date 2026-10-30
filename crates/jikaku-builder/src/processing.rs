use jikaku_bin::types::Point;

use crate::types::{RawPoint, Stroke};

pub fn linear_normalization(strokes: &[Stroke<RawPoint>], grid_size: u8) -> Vec<Stroke<Point>> {
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = 0.0;
    let mut max_y = 0.0;

    for point in strokes.iter().flat_map(|s| &s.points) {
        min_x = point.x.min(min_x);
        min_y = point.y.min(min_y);
        max_x = point.x.max(max_x);
        max_y = point.y.max(max_y);
    }

    let width = max_x - min_x;
    let height = max_y - min_y;
    let vratio = grid_size as f32 / width;
    let hratio = grid_size as f32 / height;
    let ratio = vratio.min(hratio);

    strokes
        .iter()
        .map(|s| map_stroke(s, min_x, min_y, ratio))
        .collect()
}

fn map_stroke(stroke: &Stroke<RawPoint>, x_start: f32, y_start: f32, ratio: f32) -> Stroke<Point> {
    let points = stroke
        .points
        .iter()
        .map(|p| {
            let x = ((p.x - x_start) * ratio).round().min(255.0) as u8;
            let y = ((p.y - y_start) * ratio).round().min(255.0) as u8;

            Point { x, y }
        })
        .collect();

    Stroke { points }
}

pub fn euclidian_reduction(strokes: &[Stroke<Point>], min_distance: u8) -> Vec<Stroke<Point>> {
    strokes
        .iter()
        .map(|s| reduce_stroke(s, min_distance))
        .collect()
}

fn reduce_stroke(stroke: &Stroke<Point>, min_distance: u8) -> Stroke<Point> {
    let mut points = Vec::new();
    let min_distance = (min_distance as u32).pow(2);

    for (i, point) in stroke.points.iter().enumerate() {
        if i == 0 || i == stroke.points.len() - 1 {
            points.push(*point);
        } else if let Some(last) = points.last() {
            let distance =
                (last.x as i32 - point.x as i32).pow(2) + (last.y as i32 - point.y as i32).pow(2);

            if distance > min_distance as i32 {
                points.push(*point);
            }
        }
    }

    Stroke { points }
}
