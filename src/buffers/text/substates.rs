use std::default::Default;
use super::Buffer;

use super::kb;

#[derive(Debug, Default, Clone)]
pub struct InputState {
    amount: usize,
    pub substate: SubState,
    
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SubState {
    #[default]
    Default,
    Center,
    CenterCursor,
}

trait SubCommand {
    fn apply(state:&mut Buffer, key:kb::KeyCode) -> Option<kb::KeyCode>;
}
 
impl Buffer {
    
    pub fn current_substate(&self) -> SubState {
        self.state.substate
    }
    pub fn set_substate(&mut self, substate:SubState) {
        self.state.substate = substate;
    }
    
    pub fn reset_substate(&mut self) {
        self.state.substate = SubState::Default;
    }
}


impl InputState {
    pub fn get_movement_modifier(&mut self) -> usize{
        let holder = self.amount+1;
        self.reset();
        holder
    }
    
    pub fn reset(&mut self) {
        self.clone_from(&Self::default());
    }
    
    pub fn add_number(&mut self, number:u8) {
        self.amount *= 10;
        let number = usize::try_from(number).unwrap();
        self.amount += number;
    }
    
}

    
impl SubState {
    pub fn apply(&self, state:&mut Buffer, key:kb::KeyCode) -> Option<kb::KeyCode>{
        match self {
            SubState::Default => {
                Some(key)
            }
            SubState::Center => {
                Center::apply(state, key)
            }
            SubState::CenterCursor => {
                CenterCursor::apply(state, key)
            }
        }
    }
}

pub struct CenterCursor{}
impl SubCommand for CenterCursor {
    fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> Option<kb::KeyCode>{
        match key {
            kb::KeyCode::Letter(b't') => {
                let holder = buffer.cursor.set_top_cursor(buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            kb::KeyCode::Letter(b'z') => {
                let holder = buffer.cursor.center_cursor(buffer.size(), buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            kb::KeyCode::Letter(b'b') => {
                let holder = buffer.cursor.set_bottom_cursor(buffer.size(), buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            _ => {
                Some(key)
            }
        }
    }
}


pub struct Center{}
impl SubCommand for Center {
    fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> Option<kb::KeyCode>{
        match key {
            kb::KeyCode::Letter(b't') => {
                let holder = buffer.cursor.set_top_cursor(buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            kb::KeyCode::Letter(b'z') => {
                let holder = buffer.cursor.center_cursor(buffer.size(), buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            kb::KeyCode::Letter(b'b') => {
                let holder = buffer.cursor.set_bottom_cursor(buffer.size(), buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                buffer.reset_substate();
                None
            }
            _ => {
                Some(key)
            }
        }
    }
}

