// Header (11 bytes)
// ----------------
// 0x00-0x03: magic 'KAKU'           (4)
// 0x04-0x05: character count        (2)
// 0x06: maximum stroke count        (1)
// 0x07-0x08: grid size              (2)
// 0x09: curve steps                 (1)
// 0x0A: minimum point distance      (1)
//
// Body (variable length)
// ----------------------
// - stroke count to character offset map (6 * n) {index: 2, count: 2}
// - character entry list (9 * n)
// - stroke entry list (5 * n)
// - point list (2 * n)

pub const HEADER_SIZE: usize = 11;
pub const CHAR_ID_SIZE: usize = 4;
pub const CHAR_MAP_ENTRY_SIZE: usize = 4;
pub const CHAR_ENTRY_SIZE: usize = 9;
pub const STROKE_ENTRY_SIZE: usize = 5;
pub const POINT_ENTRY_SIZE: usize = 2;
