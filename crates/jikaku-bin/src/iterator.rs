use crate::{reader::BinaryReader, types::Character};

#[derive(Debug)]
pub struct CharacterIterator<'a> {
    pub index: u16,
    pub remainder: u16,
    pub reader: &'a BinaryReader<'a>,
}

impl Iterator for CharacterIterator<'_> {
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
