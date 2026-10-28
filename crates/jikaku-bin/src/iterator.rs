use crate::{
    encoding::{POINT_ENTRY_SIZE, STROKE_ENTRY_SIZE},
    reader::BinaryReader,
    types::{Character, Point, Stroke},
};

#[derive(Debug)]
pub struct CharacterIterator<'r, 'a> {
    pub index: u16,
    pub remainder: u16,
    pub reader: &'r BinaryReader<'a>,
}

impl Iterator for CharacterIterator<'_, '_> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder == 0 {
            return None;
        }

        let value = self.reader.get_char_by_index(self.index);

        self.index += 1;
        self.remainder -= 1;

        Some(value)
    }
}

#[derive(Debug)]
pub struct StrokeIterator<'r, 'a> {
    pub offset: u32,
    pub remainder: u8,
    pub reader: &'r BinaryReader<'a>,
}

impl Iterator for StrokeIterator<'_, '_> {
    type Item = Stroke;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder == 0 {
            return None;
        }

        let value = self.reader.get_stroke(self.offset as usize);

        self.offset += STROKE_ENTRY_SIZE as u32;
        self.remainder -= 1;

        Some(value)
    }
}

#[derive(Debug)]
pub struct PointIterator<'r, 'a> {
    pub offset: u32,
    pub remainder: u8,
    pub reader: &'r BinaryReader<'a>,
}

impl Iterator for PointIterator<'_, '_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder == 0 {
            return None;
        }

        let value = self.reader.get_point(self.offset as usize);

        self.offset += POINT_ENTRY_SIZE as u32;
        self.remainder -= 1;

        Some(value)
    }
}
