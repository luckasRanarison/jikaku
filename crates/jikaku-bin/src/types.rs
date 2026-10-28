#[derive(Debug)]
pub struct CharacterRef {
    pub id: u32,
    pub stroke_count: u8,
    pub stroke_offset: u32,
}

#[derive(Debug)]
pub struct StrokeRef {
    pub point_offset: u32,
    pub point_count: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}
