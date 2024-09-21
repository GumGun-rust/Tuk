mod file_explorer_buffer;
//mod text_buffer;
mod text;
use text::Buffer as TextBuffer;
mod traits;
pub use traits::*;

use super::kb;
use super::h_s::TPos;
use super::BufferError;

use arrayvec::ArrayString;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Buffers {
    //Text2(text_buffer::Buffer),
    Text(TextBuffer),
    FileExp(file_explorer_buffer::Buffer),
}

impl std::ops::Deref for Buffers {
    type Target = dyn BufferTraits;
    fn deref(&self) -> &Self::Target {
        match self{
            Self::Text(text_buffer) => {text_buffer}
            _ => {panic!();}
        }
    }
}

impl std::ops::DerefMut for Buffers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self{
            Self::Text(text_buffer) => {text_buffer}
            _ => {panic!();}
        }
    }
}

impl Buffers {
    #[allow(unused_variables)]
    pub fn new_text(offset:TPos<u16>, term_size:TPos<u16>, opening_file:Option<&str>) -> Self {
        Buffers::Text(TextBuffer::new(offset, term_size, opening_file))
    }
}

/*
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
    fn get_cursor_location(&self) -> (TPos<u16>, char) {
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


/*
impl MoveWindow for Buffers {
    fn move_window(&mut self, arg:TPos<u16>) ->Result<(), ()> {
        match self {
            Buffers::Text(buffer) => {
                buffer.move_window(arg)
            }
            Buffers::FileExp(buffer) => {
                todo!("{:?}", buffer);
            }
        }
    }
}
*/

*/


#[derive(Default, Debug, Clone)]
pub struct StatusBarData {
    pub mode_color: ArrayString<32>,
    pub mode_text: ArrayString<32>,
    pub file_color: ArrayString<32>,
    pub middle_color: ArrayString<32>,
}


