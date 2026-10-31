use crate::reader::BinaryReader;

#[derive(Debug)]
pub struct CharacterRef<'a> {
    pub id: u32,
    pub stroke_count: u8,
    pub stroke_offset: u32,
    pub reader: &'a BinaryReader<'a>,
}

impl CharacterRef<'_> {
    pub fn get_stroke(&self, index: usize) -> StrokeRef<'_> {
        self.reader.get_stroke_at(index, self)
    }

    pub fn strokes(&self) -> impl ExactSizeIterator<Item = StrokeRef<'_>> {
        (0..self.stroke_count)
            .into_iter()
            .map(|i| self.get_stroke(i as usize))
    }
}

#[derive(Debug)]
pub struct StrokeRef<'a> {
    pub point_offset: u32,
    pub point_count: u8,
    pub reader: &'a BinaryReader<'a>,
}

impl StrokeRef<'_> {
    pub fn get_point(&self, index: usize) -> Point {
        self.reader.get_point_at(index, self)
    }

    pub fn points(&self) -> impl ExactSizeIterator<Item = Point> {
        (0..self.point_count)
            .into_iter()
            .map(|i| self.get_point(i as usize))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}
