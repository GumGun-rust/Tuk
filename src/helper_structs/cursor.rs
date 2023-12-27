

#[derive(Debug, Default, Clone, Copy)]
pub enum CursorType {
    #[default]
    Block0,
    Block1,
    Block2,
    Underline0,
    Underline1,
    Line0,
    Line1,
    Glyph0,
    Glyph1,
}


impl From<CursorType> for char {
    fn from(base:CursorType) -> Self {
        use CursorType::*;
        match base {
            Block0 => '0',
            Block1 => '1',
            Block2 => '2',
            Underline0 => '3',
            Underline1 => '4',
            Line0 => '5',
            Line1 => '6',
            Glyph0 => '7',
            Glyph1 => '8',
        }
    }
}
