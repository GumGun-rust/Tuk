use super::{
    super::{
        h_s::{
            TPos,
        },
        kb,
    },
    StatusBarData,
};


pub trait ProcessKey {
    fn process_key(&mut self, key:kb::KeyCode);
}

pub trait GetCursorLocation {
    fn get_cursor_location(&self) -> (TPos<u16>, char);
}
    
pub trait GetVisualBuffer {
    fn get_visual_buffer(&self) -> &str;
}
    
pub trait GetSbData {
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>);
}

pub trait UpdateVisualBuffer {
    fn update_visual_buffer(&mut self);
}
