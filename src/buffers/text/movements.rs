
use std::cmp;
use super::super::super::h_s::TPos;
use super::helper::CommandModifiers;

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

#[derive(Debug)]
pub enum VertDirection {
    Up,
    Down,
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
        //use MoveAction::*;
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
    
    pub(super) fn calculate_up(&self, modifier:CommandModifiers) -> VertMove {
        let movement = cmp::min(self.doc_position.rows, modifier.amount);
        VertMove::push_doc(VertDirection::Up, movement)
    }
    
    pub(super) 
    fn calculate_up_pass(
        &self, 
        modifier: CommandModifiers, 
        scrolloff: u16
    ) -> VertMove { 
        
        let cursor_rows = usize::try_from(self.doc_cursor_visual.rows-scrolloff).expect(MESSAGE);
        let mut left_amount = modifier.amount;
        let pass_cursor_limit = cmp::min(cmp::min(self.doc_position.rows, cursor_rows), left_amount);
        left_amount -= pass_cursor_limit;
        /*
        eprintln!("=================");
        eprintln!("pass-cursor:{}", pass_cursor_limit);
        eprintln!("left:{}", left_amount);
        */
        let push_pass_limit = cmp::min(cursor_rows-pass_cursor_limit, left_amount);
        left_amount -= push_pass_limit;
        /*
        eprintln!("push-pass:{}", push_pass_limit);
        eprintln!("left:{}", left_amount);
        */
        let push_doc_limit = cmp::min(self.doc_position.rows-pass_cursor_limit, left_amount);
        /*
        eprintln!("push-doc:{}", push_doc_limit);
        eprintln!("left:{}", left_amount);
        */
        
        VertMove{
            direction: VertDirection::Up,
            push_doc: push_doc_limit,
            push_pass: push_pass_limit,
            pass_cursor: pass_cursor_limit,
        }
    }
    
    pub(super) 
    fn center_cursor(
        &self, 
    ) -> VertMove {
        let buffer_size = isize::try_from(self.buffer_size.rows).expect(MESSAGE)/2;
        let cursor_rows = isize::try_from(self.doc_cursor_visual.rows).expect(MESSAGE);
        
        let movement = buffer_size - cursor_rows;
        
        match movement {
            amount @ isize::MIN ..= -1 => {
                let holder = CommandModifiers{
                    amount: amount.abs() as usize //abs of isize is granted to be a usize
                };
                self.calculate_up_pass(holder, 0)
            }
            amount @ 1 ..= isize::MAX => {
                todo!("subir {amount}");
            }
            _ => {
                VertMove::empty()
            }
        }
    }
    
    pub(super) fn calculate_down(&self, doc_size:usize) -> Option<VertMove> {
        if self.doc_position.rows < doc_size-1 {
            Some(VertMove::push_doc(VertDirection::Down, 1))
        } else {
            None
        }
    }
    
    pub(super) fn calculate_down_pass(&self) -> VertMove {
        VertMove{
            direction: VertDirection::Down,
            push_doc: 0,
            push_pass: 0,
            pass_cursor: 1,
        }
    }
}




