use super::{
    super::{
        super::{
            h_s::{
                TPos,
            },
        }
    },
    CursorSelector,
};

#[derive(Default, Debug)]
pub(in crate::buffers::text_buffer) struct Cursor {
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
    
    //amount of margin line on move operations
    pub scrollof: u16,
    
    //snap to endline
    pub endline:bool,
    
    
}

pub enum BackspaceAction {
    DeleteChar,
    FuseLine,
    None,
}

impl Cursor {
    
    pub(super) fn new(offset:TPos<u16>, term_size:TPos<u16>, doc_offset:TPos<usize>) -> Self {
        Self{
            offset: offset,
            buffer_size: term_size,
            doc_offset: doc_offset,
            scrollof: 2,
            ..Cursor::default()
        }
    }


    pub(super) fn sync(&mut self, sync_cursor:CursorSelector) {
        match sync_cursor {
            CursorSelector::Main => {
                self.sec_doc_position = self.doc_position.cols;
                self.sec_doc_cursor_visual = self.doc_cursor_visual.cols;
            }
            CursorSelector::Secondary => {
                self.doc_position.cols = self.sec_doc_position;
                self.doc_cursor_visual.cols = self.sec_doc_cursor_visual;
            }
        }
    }
    
    /*
    pub(super) fn move_rigth_visual(&mut self, line_len:usize) {
        if  line_len > 1 && self.doc_position.cols <= line_len-2 {
            self.doc_position.cols += 1;
            self.sec_doc_position += 1;
            self.doc_cursor_visual.cols += 1;
            self.sec_doc_cursor_visual += 1;
        }
    }
    
    pub(super) fn move_rigth_insert(&mut self, line_len:usize) {
        if  line_len > 1 && self.doc_position.cols <= line_len-1 {
            self.doc_position.cols += 1;
            self.sec_doc_position += 1;
            self.doc_cursor_visual.cols += 1;
            self.sec_doc_cursor_visual += 1;
        }
    }
    
    pub(super) fn move_left_visual(&mut self) {
        self.move_left();
    }

    pub(super) fn move_left_insert(&mut self) {
        self.move_left();
    }

    #[inline(always)]
    fn move_left(&mut self) {
        if self.doc_position.cols != 0 {
            self.doc_position.cols -= 1;
            self.sec_doc_position -= 1;
            self.doc_cursor_visual.cols -= 1;
            self.sec_doc_cursor_visual -= 1;
        }
    }
    */
    
    pub(super) fn enter_movement(&mut self, margin_left:u8) {
        self.doc_position.rows += 1; 
        self.doc_position.cols = 0; 
        self.sec_doc_position = 0;
        self.doc_cursor_visual.cols = u16::from(margin_left); 
        //self.doc_cursor_visual.rows += 1;
        self.sec_doc_cursor_visual = self.doc_cursor_visual.cols; 
        self.doc_offset.rows += 1;
        //up::action_push(self, 1);
    }
    
    pub(super) fn backspace_movement(&mut self, margin_left:u8, prev_line_len:usize) -> BackspaceAction {
        if self.doc_position.cols > 0 {
            self.doc_position.cols -= 1; 
            self.sec_doc_position -= 1; 
            self.doc_cursor_visual.cols -= 1; 
            self.sec_doc_cursor_visual -= 1; 
            BackspaceAction::DeleteChar
        } else {
            if self.doc_position.rows > 0 {
                self.doc_position.rows -= 1; 
                self.doc_cursor_visual.rows -= 1;
                self.doc_position.cols = prev_line_len;
                self.sec_doc_position = self.doc_position.cols;
                self.doc_cursor_visual.cols = u16::from(margin_left)+u16::try_from(prev_line_len).expect("long line not supported"); 
                self.sec_doc_cursor_visual = self.doc_cursor_visual.cols;
                BackspaceAction::FuseLine
            } else {
                BackspaceAction::None
            }
        }
    }
    
    #[inline(always)]
    pub(super) fn insert_char_movement(&mut self) {
        self.doc_position.cols += 1; 
        self.sec_doc_position += 1; 
        self.doc_cursor_visual.cols += 1; 
        self.sec_doc_cursor_visual += 1; 
    }

}


pub(in crate::buffers::text_buffer) 
mod up {
    use super::*;
    
    #[derive(Debug)]
    pub(in crate::buffers::text_buffer) 
    struct MoveQuant {
        move_amnt:usize,
        move_push_amnt:usize,
        push_amnt:usize,
    }
    
    /*
    #[allow(dead_code)]
    #[derive(Debug)]
    pub(in crate::buffers::text_buffer) 
    enum MoveType {
        Move(usize),
        Push(usize),
        MoveAndPush(usize),
        None,
    }
    */
    
    /*TODO use amount*/
    
    
    pub(in crate::buffers::text_buffer) 
    mod visual{
        use super::*;
        
        /*
        pub(in crate::buffers::text_buffer) 
        fn new_can_move(buffer:&mut Cursor, line_count:usize, amount:usize) -> MoveQuant {
            //move calc block
            
            if buffer.doc_position.rows < amount {
                
                
            }
            
            MoveQuant{
                move_amnt:0,
                move_push_amnt:0,
                push_amnt:0,
            }
        }
        */
        
        
        /*
        pub(in crate::buffers::text_buffer) 
        fn can_move(buffer:&mut Cursor, line_count:usize, amount:usize) -> MoveType {
            
            match (buffer.doc_position.rows, buffer.doc_offset.rows) {
                (_, _) => {
                    return MoveType::MoveAndPush(1);
                }
            }
            panic!("up");
            /*
            if buffer.doc_offset.rows < usize::from(buffer.buffer_size.rows - buffer.scrollof)  {
                
                if buffer.doc_position.rows == 0 {
                    
                    return MoveType::MoveAndPush(amount);
                }
                return MoveType::Move(amount);
            }
            if buffer.doc_position.rows+1 == line_count {
                return MoveType::None;
                
            } 
            return MoveType::Push(amount);
            */
        }
        */
    }
    
    /*
    pub(in crate::buffers::text_buffer) 
    fn action_move(buffer:&mut Cursor, amount:usize) {
        buffer.doc_cursor_visual.rows -= u16::try_from(amount).expect("cant be bigger than buffer size");
        buffer.doc_position.rows -= amount;
    }
    */
    
    /*
    pub(in crate::buffers::text_buffer) 
    fn action_move_and_push(buffer:&mut Cursor, amount:usize) {
        buffer.doc_offset.rows += 1;
        buffer.doc_cursor_visual.rows -= 1;
    }
    */
    
    /*
    pub(in crate::buffers::text_buffer) 
    fn action_push(buffer:&mut Cursor, amount:usize) {
        buffer.doc_offset.rows += 1;
        buffer.doc_position.rows += 1;
    }
    */
    
}

pub(in crate::buffers::text_buffer) 
mod down {
    use super::*;
    
    #[allow(dead_code)]
    #[derive(Debug)]
    pub(in crate::buffers::text_buffer) 
    enum MoveType {
        Move(usize),
        Push(usize),
        MoveAndPush(usize),
        None,
    }
    
    /*TODO use amount*/
    
    pub(in crate::buffers::text_buffer) 
    mod visual{
        use super::*;
        
        pub(in crate::buffers::text_buffer) 
        fn can_move(buffer:&mut Cursor, line_count:usize, amount:usize) -> MoveType {
            
            
            match (buffer.doc_position.rows, buffer.doc_offset.rows) {
                (position, offset) if position == 0 && offset == buffer.scrollof.into() => {
                    return MoveType::None;
                }
                (position, offset) if offset-position == buffer.scrollof.into() => {
                    return MoveType::Push(1);
                }
                (position, offset) if position == line_count-1 => {
                    return MoveType::MoveAndPush(1);
                }
                (_, _) => {
                    return MoveType::Move(1);
                }
            }
        }
    }
    
    pub(in crate::buffers::text_buffer) 
    fn action_move(buffer:&mut Cursor, amount:usize) {
        buffer.doc_cursor_visual.rows += 1;
        buffer.doc_position.rows += 1;
    }
    
    pub(in crate::buffers::text_buffer) 
    fn action_move_and_push(buffer:&mut Cursor, amount:usize) {
        buffer.doc_offset.rows -= 1;
        buffer.doc_cursor_visual.rows += 1;
    }
    
    pub(in crate::buffers::text_buffer) 
    fn action_push(buffer:&mut Cursor, amount:usize) {
        buffer.doc_offset.rows -= 1;
        buffer.doc_position.rows -= 1;
    }
    
}

