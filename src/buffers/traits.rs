use super::{
    super::{
        h_s::{
            TPos,
        },
        kb,
    },
    StatusBarData,
};

pub trait BufferTraits: ProcessKey + GetCursorLocation + GetSbData + GetVisualBuffer/* + UpdateVisualBuffer*/ + MoveWindow/* + ResizeWindow*/{
    
}

impl<BufferType:ProcessKey + GetCursorLocation + GetSbData + GetVisualBuffer/* + UpdateVisualBuffer*/ + MoveWindow/* + ResizeWindow*/> BufferTraits for BufferType{
    
}

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


pub trait ResizeWindow {
    //Will move the window using the top left corner as the pivot
    fn resize_window(&mut self, _:TPos<u16>);
    
    fn get_size(&self) -> TPos<u16>;
}

pub trait MoveWindow {
    //Will move the window using the top left corner as the pivot
    fn move_window(&mut self, _:TPos<u16>) -> Result<(), ()>;
    
    fn get_position(&self) -> TPos<u16>;
}

/*
pub trait MoveResize: MoveWindow + ResizeWindow {
    fn move_resize_window(&mut self);
}
*/
