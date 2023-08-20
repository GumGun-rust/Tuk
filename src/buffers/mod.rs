mod file_explorer_buffer;
mod text_buffer;
mod traits;
pub use traits::*;

use super::{
    kb,
    h_s::{
        TPos,
    },
};

use arrayvec::ArrayString;

#[derive(Debug)]
pub enum Buffers {
    Text(text_buffer::Buffer),
    FileExp(file_explorer_buffer::Buffer),
}


impl Buffers {
    pub fn new_text(offset:TPos<u16>, term_size:TPos<u16>, opening_file:Option<&str>) -> Self {
        Buffers::Text(text_buffer::Buffer::new(offset, term_size, opening_file))
    }
}

impl ProcessKey for Buffers {
    fn process_key(&mut self, key:kb::KeyCode){
        match self {
            Buffers::Text(buffer) => {
                buffer.process_key(key)
            }
            Buffers::FileExp(buffer) => {
                todo!("{:?}", buffer);
            }
        }
    }
}



impl GetCursorLocation for Buffers {
    fn get_cursor_location(&self) -> TPos<u16> {
        match self {
            Buffers::Text(buffer) => {
                buffer.get_cursor_location()
            }
            Buffers::FileExp(buffer) => {
                todo!("{:?}", buffer);
            }
        }
    }
}



impl GetVisualBuffer for Buffers {
    fn get_visual_buffer(&self) -> &str{
        match self {
            Buffers::Text(buffer) => {
                buffer.get_visual_buffer()
            }
            Buffers::FileExp(buffer) => {
                todo!("{:?}", buffer);
            }
        }
    }
}



impl GetSbData for Buffers {
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>){
        match self {
            Buffers::Text(buffer) => {
                buffer.get_sb_data()
            }
            Buffers::FileExp(buffer) => {
                todo!("{:?}", buffer);
            }
        }
    }
}



#[derive(Default, Debug, Clone)]
pub struct StatusBarData {
    pub mode_color: ArrayString<32>,
    pub mode_text: ArrayString<32>,
    pub file_color: ArrayString<32>,
    pub middle_color: ArrayString<32>,
}

