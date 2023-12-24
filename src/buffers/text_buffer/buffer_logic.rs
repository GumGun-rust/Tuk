use super::Buffer;

use super::{
    super::{
        super::{
            kb,
        }
    },
    movements,
    EditorMode,
};

impl Buffer {
    
    pub(super) fn insert_char(&mut self, letter:char) {
        self.lines.insert_char(self.test_cursor.doc_position, letter).unwrap(); 
        self.test_cursor.insert_char_movement();
        self.update_visual_buffer();
    }

    
    pub(super) fn move_cursor_up_visual(&mut self) {
        use movements::up;
        /*
        match up::visual::can_move(&mut self.test_cursor, self.lines.len(), 1) {
            up::MoveType::Push(amount) => {
                up::action_push(&mut self.test_cursor, amount);
            }
            up::MoveType::MoveAndPush(amount) => {
                up::action_push(&mut self.test_cursor, amount);
                up::action_move(&mut self.test_cursor, amount);
                //up::action_move_and_push(&mut self.test_cursor, amount);
            }
            up::MoveType::Move(amount) => {
                up::action_move(&mut self.test_cursor, amount);
            }
            up::MoveType::None => {}
        }
        */
        panic!();
    }
    
    pub(super) fn move_cursor_down_visual(&mut self) {
        /*
        use movements::down;
        match down::visual::can_move(&mut self.test_cursor, self.lines.len(), 1) {
            down::MoveType::None => {}
            down::MoveType::Push(amount) => {
                down::action_push(&mut self.test_cursor, amount);
            }
            down::MoveType::MoveAndPush(amount) => {
                down::action_move_and_push(&mut self.test_cursor, amount);
            }
            down::MoveType::Move(amount) => {
                down::action_move(&mut self.test_cursor, amount);
            }
        }
        */
    }

    pub(super) fn move_doc(&mut self, arrow:kb::Arrow) -> Option<()> {
        match arrow {
            kb::Arrow::Up => {
                self.test_cursor.doc_offset.rows += 1;
                self.test_cursor.doc_position.rows += 1;
                /*
                let number = i64::try_from(self.test_cursor.doc_offset.rows).unwrap()-i64::try_from(self.lines.len()).unwrap();
                if number+1 < self.test_cursor.buffer_size.rows.into() {
                    self.test_cursor.doc_offset.rows += 1;
                    //self.test_cursor.doc_cursor_visual.rows -= 1;
                }
                */
            }
            kb::Arrow::Down => {
                self.test_cursor.doc_offset.rows -= 1;
                self.test_cursor.doc_position.rows -= 1;
                
            }
            kb::Arrow::Left => {
                self.test_cursor.doc_offset.cols -= 1;
                
            }
            kb::Arrow::Right => {
                self.test_cursor.doc_offset.cols += 1;
            }
        }
        self.update_cursor_location();
        None
    }



    pub(super) fn update_cursor_location(&mut self) {
        self.test_cursor.doc_cursor_visual;
        /*
        self.test_cursor.doc_cursor_visual = TPos::<u16>{
            cols: u16::from(self.margin_left)+u16::try_from(self.test_cursor.doc_position.cols).unwrap(),
            rows: u16::try_from(self.test_cursor.buffer_size.rows).unwrap() - (u16::try_from(self.test_cursor.doc_offset.rows).unwrap() - u16::try_from(self.test_cursor.doc_position.rows).unwrap())
        };
        */
    }

    
    pub(super) fn enter_insert_mode_after(&mut self)  {
        self.mode = EditorMode::Insert;
        self.cursor_type = '6';
        if self.lines[self.test_cursor.doc_position.rows].len() > self.test_cursor.doc_position.cols {
            self.test_cursor.doc_position.cols += 1;
            self.test_cursor.doc_cursor_visual.cols += 1;
        } 
    }
    
    pub(super) fn enter_insert_mode(&mut self)  {
        self.mode = EditorMode::Insert;
        self.cursor_type = '6';
    }
    
    pub(super) fn delete_char(&mut self) {
        self.lines.delete_char(self.test_cursor.doc_position).unwrap();
    }
    
    pub(super) fn insert_empty_line_bellow(&mut self) {
        self.lines.insert_line(self.test_cursor.doc_position.rows+1, "".to_owned()).unwrap();
        
        self.test_cursor.doc_position.rows += 1; 
        self.test_cursor.doc_cursor_visual.rows += 1;
        
        self.test_cursor.doc_position.cols = 0; 
        self.test_cursor.doc_cursor_visual.cols = u16::from(self.margin_left); 
        
        self.enter_insert_mode();
        self.update_visual_buffer();
    }
    
    
    pub(super) fn insert_empty_line_above(&mut self) {
        self.lines.insert_line(self.test_cursor.doc_position.rows, "".to_owned()).unwrap();
        
        self.test_cursor.doc_position.cols = 0; 
        self.test_cursor.doc_cursor_visual.cols = u16::from(self.margin_left); 
        
        self.enter_insert_mode();
        self.update_visual_buffer();
    }
    
    
}
