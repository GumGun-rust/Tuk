
use std::cmp;
use super::super::super::h_s::TPos;
//use super::helper::CommandModifiers;

#[allow(dead_code)]
#[derive(Default, Debug)]
enum CursorSelector {
    #[default]
    Main,
    Secondary,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub(in super::super) struct Cursor {
    //location of the upper left corner of the buffer relative to the terminal,
    pub offset: TPos<u16>,
    
    //size from the upper left corner to the bottom right
    pub buffer_size: TPos<u16>,
    
    //location from the upper left corner of the buffer to the upper left corner of the document
    pub doc_offset: TPos<usize>,
    
    //absolute address of the cursor in the document
    pub doc_position: TPos<usize>,
    
    //visual position of the cursor relative to the offset
    pub doc_cursor_visual: TPos<u16>,
    
    //address of the secundary cursor
    pub sec_doc_position: usize,
    
    //visual position of the relative cursor
    pub sec_doc_cursor_visual: u16,
    
    //snap to endline
    pub endline:bool,
    
}

const MESSAGE:&'static str = "In normal arch this should never fail";

/*
pub enum BackspaceAction {
    DeleteChar,
    FuseLine,
    None,
}
*/

#[derive(Debug, Clone, Copy)]
pub enum VertDirection {
    Up,
    Down,
}

impl VertDirection{
    fn complementary(self) -> VertDirection {
        use VertDirection::*;
        match self {
            Up => Down,
            Down => Up,
        }
    }
}

#[derive(Debug)]
pub(super)
struct VertMove {
    direction: VertDirection, 
    //action: MoveAction,
    push_doc: usize,
    push_pass: usize,
    pass_cursor: usize,
}

/*
#[derive(Debug)]
enum MoveAction {
    Pass(usize),
    PushPass(usize,usize),
}

#[derive(Debug, Clone)]
struct MoveAction {
    push_doc: usize,
    push_pass: usize,
    pass_cursor: usize,
}
*/

impl VertMove {
    fn push_doc(direction:VertDirection, ammnt:usize) -> Self {
        Self{
            direction: direction,
            push_doc: ammnt,
            push_pass: 0,
            pass_cursor: 0,
        }
    }
    fn empty() -> Self {
        Self{
            direction: VertDirection::Up,
            push_doc: 0,
            push_pass: 0,
            pass_cursor: 0,
        }
    }
    
    pub fn get_complementary(&self) -> Self {
        
        Self{
            direction: self.direction.complementary(),
            push_doc: self.pass_cursor,
            push_pass: 0,
            pass_cursor: 0,
        }
        
    }
    
}

impl Cursor {
    
    pub(super) fn new(offset:TPos<u16>, term_size:TPos<u16>, doc_offset:TPos<usize>) -> Self {
        Self{
            offset: offset,
            buffer_size: term_size,
            doc_offset: doc_offset,
            ..Cursor::default()
        }
    }
    
    pub(super) fn apply(&mut self, movement:VertMove) {
        use VertDirection::*;
        match movement.direction {
            Down => {
                self.doc_offset.rows += movement.push_doc;
                self.doc_position.rows += movement.push_doc;
                
                self.doc_offset.rows -= movement.push_pass;
                self.doc_position.rows -= movement.push_pass;
                
                self.doc_cursor_visual.rows += u16::try_from(movement.pass_cursor+movement.push_pass).expect("should be validated");
                self.doc_position.rows += movement.pass_cursor+movement.push_pass;
            }
            Up => {
                
                self.doc_offset.rows -= movement.push_doc;
                self.doc_position.rows -= movement.push_doc;
                
                self.doc_offset.rows += movement.push_pass;
                self.doc_position.rows += movement.push_pass;
                
                self.doc_cursor_visual.rows -= u16::try_from(movement.pass_cursor+movement.push_pass).expect("should be validated");
                self.doc_position.rows -= movement.pass_cursor+movement.push_pass;
            }
        }
    }
    
    pub(super) fn calculate_up(
        &self, 
        modifier:usize
    ) -> VertMove {
        let movement = cmp::min(self.doc_position.rows, modifier);
        VertMove::push_doc(VertDirection::Up, movement)
    }
    
    pub(super) fn calculate_down(
        &self, 
        doc_size:usize,
        modifier: usize,
    ) -> VertMove {
        
        let lines_left = doc_size - self.doc_position.rows - 1;
        let movement = cmp::min(lines_left, modifier);
        VertMove::push_doc(VertDirection::Down, movement)
    }
    
    pub(super) 
    fn calculate_up_pass(
        &self, 
        modifier: usize,
        scrolloff: u16,
    ) -> VertMove { 
        
        let cursor_rows = usize::try_from(self.doc_cursor_visual.rows-scrolloff).expect(MESSAGE);
        let mut left_amount = modifier;
        let pass_cursor_limit = cmp::min(cmp::min(self.doc_position.rows, cursor_rows), left_amount);
        left_amount -= pass_cursor_limit;
        let push_pass_limit = cmp::min(cursor_rows-pass_cursor_limit, left_amount);
        left_amount -= push_pass_limit;
        let push_doc_limit = cmp::min(self.doc_position.rows-pass_cursor_limit, left_amount);
        
        VertMove{
            direction: VertDirection::Up,
            push_doc: push_doc_limit,
            push_pass: push_pass_limit,
            pass_cursor: pass_cursor_limit,
        }
    }
    
    pub(super) fn calculate_down_pass(
        &self,
        doc_size:usize,
        modifier: usize,
        scrolloff: u16,
    ) -> VertMove {
        
        let lines_left = doc_size - self.doc_position.rows - 1;
        
        let cursor_rows = usize::try_from(self.buffer_size.rows - self.doc_cursor_visual.rows - scrolloff).expect(MESSAGE);
        let mut left_amount = modifier;
        let pass_cursor_limit = cmp::min(cmp::min(lines_left, cursor_rows), left_amount);
        left_amount -= pass_cursor_limit;
        
        let push_pass_limit = cmp::min(cursor_rows-pass_cursor_limit, left_amount);
        left_amount -= push_pass_limit;
        
        let push_doc_limit = cmp::min(lines_left-pass_cursor_limit, left_amount);
        
        VertMove{
            direction: VertDirection::Down,
            push_doc: push_doc_limit,
            push_pass: push_pass_limit,
            pass_cursor: pass_cursor_limit,
        }
    }
    
    pub(super) 
    fn center_cursor(
        &self, 
        doc_size:usize,
        scrolloff:u16,
    ) -> VertMove {
        let buffer_size = isize::try_from(self.buffer_size.rows).expect(MESSAGE)/2;
        let cursor_rows = isize::try_from(self.doc_cursor_visual.rows).expect(MESSAGE);
        let movement = buffer_size - cursor_rows;
        match movement {
            amount @ isize::MIN ..= -1 => {
                self.calculate_up_pass(amount.abs() as usize, scrolloff)//abs of isize is granted to be a usize
            }
            amount @ 1 ..= isize::MAX => {
                self.calculate_down_pass(doc_size, amount.abs() as usize, scrolloff)//abs of isize is granted to be a usize
            }
            _ => {
                VertMove::empty()
            }
        }
    }
    
    pub(super) 
    fn set_top_cursor(
        &self, 
        scrolloff:u16,
    ) -> VertMove {
        let cursor_rows = usize::try_from(self.doc_cursor_visual.rows).expect(MESSAGE);
        let movement = cursor_rows - scrolloff as usize;
        
        if movement == 0 {
            VertMove::empty()
        } else {
            self.calculate_up_pass(movement, scrolloff)
        }
    }
    
    pub(super) 
    fn set_bottom_cursor(
        &self, 
        doc_size:usize,
        scrolloff:u16,
    ) -> VertMove {
        let buffer_size = usize::try_from(self.buffer_size.rows).expect(MESSAGE)-scrolloff as usize;
        let cursor_rows = usize::try_from(self.doc_cursor_visual.rows).expect(MESSAGE);
        let movement = buffer_size - cursor_rows;
        
        if movement == 0 {
            VertMove::empty()
        } else {
            self.calculate_down_pass(doc_size, movement, scrolloff)
        }
    }
    
    
    pub(super) 
    fn center_cursor_page(
        &self, 
        doc_size:usize,
        scrolloff:u16,
    ) -> VertMove {
        let buffer_size = isize::try_from(self.buffer_size.rows).expect(MESSAGE)/2;
        let cursor_rows = isize::try_from(self.doc_cursor_visual.rows).expect(MESSAGE);
        let movement = buffer_size - cursor_rows;
        match movement {
            amount @ isize::MIN ..= -1 => {
                self.calculate_up_pass(amount.abs() as usize, scrolloff)//abs of isize is granted to be a usize
            }
            amount @ 1 ..= isize::MAX => {
                self.calculate_down_pass(doc_size, amount.abs() as usize, scrolloff)//abs of isize is granted to be a usize
            }
            _ => {
                VertMove::empty()
            }
        }
    }
    
    
}




