#[derive(Debug)]
pub struct Character<T> {
    pub id: u32,
    pub strokes: Vec<Stroke<T>>,
}

#[derive(Debug)]
pub struct Stroke<T> {
    pub points: Vec<Point<T>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
