use super::TPos;
use super::super::kb;
use super::StatusBarData;
use super::BufferError;

pub trait BufferTraits: ProcessKey + GetCursorLocation + GetSbData + GetVisualBuffer/* + UpdateVisualBuffer*/ + MoveResizeWindow{}

impl<BufferType:ProcessKey + GetCursorLocation + GetSbData + GetVisualBuffer/* + UpdateVisualBuffer*/ + MoveResizeWindow> BufferTraits for BufferType{
    
}

pub trait ProcessKey {
    fn process_key(&mut self, key:kb::KeyCode) -> Option<()>;
}

pub trait ProcessSpecialKey {
    fn process_special_key(&mut self) -> Result<(), ()>;
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

pub trait GetSSbData { //get secundary status bar data
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>);
}

pub trait UpdateVisualBuffer {
    fn update_visual_buffer(&mut self);
}


pub trait MoveResizeWindow {
    //Will move the window using the top left corner as the pivot
    fn move_window_delta(&mut self, _:TPos<u16>, _:TPos<i32>) -> Result<TPos<u16>, BufferError>;
    //Will resize the window space between the upper left and the bottom right
    fn resize_delta(&mut self, _:TPos<u16>, _:TPos<i32>) -> Result<TPos<u16>, BufferError>;
    
    fn get_position(&self) -> TPos<u16>;
    fn get_size(&self) -> TPos<u16>;
    
    fn move_window_position(&mut self, window_size:TPos<u16>, new_position:TPos<u16>) -> Result<(), BufferError>{
        if new_position.is_wider(window_size) || new_position.is_taller(window_size) {
            return Err(BufferError::PivotOutOfScreen);
        }
        
        let total_size = new_position+self.get_size();
        if total_size.is_wider(window_size) || total_size.is_taller(window_size) {
            return Err(BufferError::WindowDoesNotFit);
        }
        
        let current = self.get_position();
        let delta = TPos{
            rows: i32::from(new_position.rows)-i32::from(current.rows),
            cols: i32::from(new_position.cols)-i32::from(current.cols),
        };
        self.move_window_delta(window_size, delta)?;
        Ok(())
    }
    
    
    
    
    fn resize(&mut self, window_size:TPos<u16>, new_size:TPos<u16>) -> Result<(), BufferError>{
        let current = self.get_size();
        
        let delta = TPos{
            rows: i32::from(new_size.rows)-i32::from(current.rows),
            cols: i32::from(new_size.cols)-i32::from(current.cols),
        };
        self.resize_delta(window_size, delta)?;
        Ok(())
    }
    /*
    */
}

/*
pub trait MoveResize: MoveWindow + ResizeWindow {
    fn move_resize_window(&mut self);
}
*/
