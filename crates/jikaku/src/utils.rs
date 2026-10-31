use jikaku_bin::types::{Point, StrokeRef};
use jikaku_builder::types::Stroke;

pub trait PointStream {
    fn get(&self, index: usize) -> Point;
}

impl PointStream for Stroke<Point> {
    fn get(&self, index: usize) -> Point {
        self.points[index]
    }
}

impl PointStream for StrokeRef<'_> {
    fn get(&self, index: usize) -> Point {
        self.get_point(index)
    }
}
