use arrayvec::ArrayVec;

use std::default::Default;
use super::Buffer;

use super::kb;
use super::EditorMode;

use super::generic_state;

#[derive(Debug, Default, Clone, Copy)]
pub struct InputState {
    amount: usize,
    pub substate: SubstateNormal,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SubstateNormal {
    #[default]
    Base,
    Center,
    CenterCursor,
}

pub enum SubCommandReturn {
    Key(kb::KeyCode),
    SubstateNormal(SubstateNormal),
    Command,
}

impl SubCommandReturn {
    fn finish() -> Self {
        SubCommandReturn::SubstateNormal(SubstateNormal::Base)
    }
    fn substate(substate:SubstateNormal) -> Self {
        SubCommandReturn::SubstateNormal(substate)
    }
    fn key(key:kb::KeyCode) -> Self {
        SubCommandReturn::Key(key)
    }
}

trait SubCommand {
    fn apply(state:&mut Buffer, key:kb::KeyCode) -> SubCommandReturn;
}
 

impl InputState {
    pub fn get_movement_modifier(&self) -> usize {
        if self.amount == 0 {
            1
        } else {
            self.amount
        }
    }
    
    pub fn add_number(&mut self, number:u8) {
        self.amount *= 10;
        let number = usize::try_from(number).unwrap();
        self.amount += number;
    }
    
    #[inline(always)]
    pub fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> Option<()> {
        
        let mut state = buffer.get_normal_state();
        
        let key = match state.substate.apply(buffer, key) {
            SubCommandReturn::Key(_) => {
                panic!();
            }
            SubCommandReturn::SubstateNormal(substate) => {
                buffer.set_normal_substate(substate);
            }
            SubCommandReturn::Command => {
                return Some(());
            }
        };
        
        None
    }
}

    
impl SubstateNormal {
    pub fn apply(&self, state:&mut Buffer, key:kb::KeyCode) -> SubCommandReturn{
        match self {
            SubstateNormal::Base => {
                BaseState::apply(state, key)
            }
            SubstateNormal::Center => {
                Center::apply(state, key)
            }
            SubstateNormal::CenterCursor => {
                CenterCursor::apply(state, key)
            }
        }
    }
}

impl Buffer {
    fn get_normal_state(&self) -> InputState {
        if let EditorMode::Normal(input_state) = self.mode {
            input_state
        } else {
            panic!("should never get to this point");
        }
    }
    
    fn get_normal_state_mut(&mut self) -> &mut InputState {
        if let EditorMode::Normal(ref mut input_state) = &mut self.mode {
            input_state
        } else {
            panic!("should never get to this point");
        }
    }
    
    fn get_normal_substate(&self) -> SubstateNormal {
        self.get_normal_state().substate
    }
    
    fn set_normal_substate(&mut self, substate:SubstateNormal) {
        self.get_normal_state_mut().substate = substate;
    }
    
    fn move_down(&mut self) {
        let input_state = self.get_normal_state();
        generic_state::move_down(self, self.size(), input_state.get_movement_modifier());
    }

    fn move_up(&mut self) {
        let input_state = self.get_normal_state();
        generic_state::move_up(self, input_state.get_movement_modifier());
    }
    
}

pub struct BaseState{}
impl SubCommand for BaseState {
    fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> SubCommandReturn{
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    b'd' => {
                        eprintln!("{:?}", letter);
                        eprintln!("{:#?}", buffer);
                        eprintln!("{:?}", buffer.lines.len())
                    }
                    b'j' => {  
                        buffer.move_down();
                    } 
                    b'J' => {  
                        buffer.key_J();
                    } 
                    b'k' => {  
                        buffer.move_up();
                    } 
                    b'K' => {  
                        buffer.key_K();
                    } 
                    b'z' => {
                        return SubCommandReturn::SubstateNormal(SubstateNormal::Center)
                    }
                    b'Z' => {
                        return SubCommandReturn::SubstateNormal(SubstateNormal::CenterCursor)
                    }
                    b':' => {
                        return SubCommandReturn::Command;
                    }
                    _ => {}
                    
                } 
                
            }
            kb::KeyCode::Number(number) => {
                buffer.key_number(number);
            }
            _ => {panic!();}
        }
        SubCommandReturn::finish()
    }
}

pub struct CenterCursor{}
impl SubCommand for CenterCursor {
    fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> SubCommandReturn{
        eprintln!("bufff");
        match key {
            kb::KeyCode::Letter(b't') => {
                let holder = buffer.cursor.set_top_cursor(buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
                
            }
            kb::KeyCode::Letter(b'z') => {
                let holder = buffer.cursor.center_cursor(buffer.size(), buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
            }
            kb::KeyCode::Letter(b'b') => {
                let holder = buffer.cursor.set_bottom_cursor(buffer.size(), buffer.config.scrolloff);
                buffer.cursor.apply(holder);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
            }
            _ => {
                SubCommandReturn::finish()
            }
        }
    }
}


pub struct Center{}
impl SubCommand for Center {
    fn apply(buffer:&mut Buffer, key:kb::KeyCode) -> SubCommandReturn{
        match key {
            kb::KeyCode::Letter(b't') => {
                let holder = buffer.cursor.set_top_cursor(buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
            }
            kb::KeyCode::Letter(b'z') => {
                let holder = buffer.cursor.center_cursor(buffer.size(), buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
            }
            kb::KeyCode::Letter(b'b') => {
                let holder = buffer.cursor.set_bottom_cursor(buffer.size(), buffer.config.scrolloff);
                let second_move = holder.get_complementary();
                buffer.cursor.apply(holder);
                buffer.cursor.apply(second_move);
                buffer.update_visual_buffer();
                SubCommandReturn::finish()
            }
            _ => {
                SubCommandReturn::finish()
            }
        }
    }
}

