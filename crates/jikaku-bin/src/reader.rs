use crate::{
    encoding::{CHAR_ENTRY_SIZE, CHAR_MAP_ENTRY_SIZE, HEADER_SIZE},
    iterator::{CharacterIterator, PointIterator, StrokeIterator},
    types::{CharacterRef, Point, StrokeRef},
};

#[derive(Debug)]
pub struct BinaryReader<'a> {
    data: &'a [u8],
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn get_char_count(&self) -> usize {
        self.read_u16(0x4).into()
    }

    pub fn get_max_stroke(&self) -> usize {
        self.read_u8(0x6).into()
    }

    pub fn get_grid_size(&self) -> u8 {
        self.read_u8(0x7)
    }

    pub fn get_curve_points(&self) -> u8 {
        self.read_u8(0x8)
    }

    pub fn get_min_point_distance(&self) -> u8 {
        self.read_u8(0x9)
    }

    pub fn get_char_map(&self, stroke_count: usize) -> (u16, u16) {
        let offset = HEADER_SIZE + (stroke_count - 1) * CHAR_MAP_ENTRY_SIZE;
        let char_idx = self.read_u16(offset);
        let char_count = self.read_u16(offset + 2);

        (char_idx, char_count)
    }

    pub fn get_chars_by_stroke(&self, stroke_count: usize) -> Option<CharacterIterator<'_, 'a>> {
        if stroke_count > self.get_max_stroke() {
            return None;
        }

        let (char_idx, char_count) = self.get_char_map(stroke_count);

        Some(CharacterIterator {
            index: char_idx,
            remainder: char_count,
            reader: self,
        })
    }

    pub fn get_char_by_index(&self, index: u16) -> CharacterRef {
        let offset = self.get_char_block_start() + index as usize * CHAR_ENTRY_SIZE;

        CharacterRef {
            id: self.read_u32(offset),
            stroke_count: self.read_u8(offset + 4),
            stroke_offset: self.read_u32(offset + 5),
        }
    }

    pub fn get_strokes(&self, character: &CharacterRef) -> StrokeIterator<'_, 'a> {
        StrokeIterator {
            offset: character.stroke_offset,
            remainder: character.stroke_count,
            reader: self,
        }
    }

    pub fn get_points(&self, stroke: &StrokeRef) -> PointIterator<'_, 'a> {
        PointIterator {
            offset: stroke.point_offset,
            remainder: stroke.point_count,
            reader: self,
        }
    }

    pub fn get_stroke(&self, offset: usize) -> StrokeRef {
        StrokeRef {
            point_count: self.read_u8(offset),
            point_offset: self.read_u32(offset + 1),
        }
    }

    pub fn get_point(&self, offset: usize) -> Point {
        Point {
            x: self.read_u8(offset),
            y: self.read_u8(offset + 1),
        }
    }

    fn get_char_block_start(&self) -> usize {
        HEADER_SIZE + self.get_max_stroke() * CHAR_MAP_ENTRY_SIZE
    }

    fn read_u8(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    fn read_u16(&self, offset: usize) -> u16 {
        u16::from_le_bytes(self.data[offset..offset + 2].try_into().unwrap())
    }

    fn read_u32(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap())
    }
}
