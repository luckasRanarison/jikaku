#[derive(Debug)]
pub struct Character<T> {
    pub id: u32,
    pub strokes: Vec<Stroke<T>>,
}

#[derive(Debug)]
pub struct Stroke<T> {
    pub points: Vec<T>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RawPoint {
    pub x: f32,
    pub y: f32,
}
