use super::Buffer;

use super::normal_state::SubstateNormal;
//use super::movements::VertMove;
//use super::EditorMode;

impl Buffer {
    
    #[allow(non_snake_case)]
    pub fn key_J(&mut self) {
        todo!();
        /*
        let holder = self.cursor.calculate_down_pass(self.size(), self.state.get_movement_modifier(), self.config.scrolloff);
        self.cursor.apply(holder);
        self.update_visual_buffer();
        */
    }
    
    #[allow(non_snake_case)]
    pub fn key_K(&mut self) {
        todo!();
        /*
        let holder = self.cursor.calculate_up_pass(self.state.get_movement_modifier(), self.config.scrolloff);
        self.cursor.apply(holder);
        self.update_visual_buffer();
        */
    }
    
    pub fn key_z(&mut self) {
    }
    
    #[allow(non_snake_case)]
    pub fn key_Z(&mut self) {
    }
    
    pub fn key_number(&mut self, number:u8) {
        todo!("");
        //self.state.add_number(number);
        //eprintln!("{:?}", self.state);
    }
    
    /*
    pub(super) fn insert_char(&mut self, letter:char) {
        self.lines.insert_char(self.cursor.doc_position, letter).unwrap(); 
        self.cursor.insert_char_movement();
        self.update_visual_buffer();
    }

    
    pub(super) fn move_cursor_up_visual(&mut self) {
        use movements::up;
        /*
        match up::visual::can_move(&mut self.cursor, self.lines.len(), 1) {
            up::MoveType::Push(amount) => {
                up::action_push(&mut self.cursor, amount);
            }
            up::MoveType::MoveAndPush(amount) => {
                up::action_push(&mut self.cursor, amount);
                up::action_move(&mut self.cursor, amount);
                //up::action_move_and_push(&mut self.cursor, amount);
            }
            up::MoveType::Move(amount) => {
                up::action_move(&mut self.cursor, amount);
            }
            up::MoveType::None => {}
        }
        */
        panic!();
    }
    
    pub(super) fn move_cursor_down_visual(&mut self) {
        /*
        use movements::down;
        match down::visual::can_move(&mut self.cursor, self.lines.len(), 1) {
            down::MoveType::None => {}
            down::MoveType::Push(amount) => {
                down::action_push(&mut self.cursor, amount);
            }
            down::MoveType::MoveAndPush(amount) => {
                down::action_move_and_push(&mut self.cursor, amount);
            }
            down::MoveType::Move(amount) => {
                down::action_move(&mut self.cursor, amount);
            }
        }
        */
    }

    pub(super) fn move_doc(&mut self, arrow:kb::Arrow) -> Option<()> {
        match arrow {
            kb::Arrow::Up => {
                self.cursor.doc_offset.rows += 1;
                self.cursor.doc_position.rows += 1;
                /*
                let number = i64::try_from(self.cursor.doc_offset.rows).unwrap()-i64::try_from(self.lines.len()).unwrap();
                if number+1 < self.cursor.buffer_size.rows.into() {
                    self.cursor.doc_offset.rows += 1;
                    //self.cursor.doc_cursor_visual.rows -= 1;
                }
                */
            }
            kb::Arrow::Down => {
                self.cursor.doc_offset.rows -= 1;
                self.cursor.doc_position.rows -= 1;
                
            }
            kb::Arrow::Left => {
                self.cursor.doc_offset.cols -= 1;
                
            }
            kb::Arrow::Right => {
                self.cursor.doc_offset.cols += 1;
            }
        }
        self.update_cursor_location();
        None
    }



    */
    pub(super) fn update_cursor_location(&mut self) {
        self.cursor.doc_cursor_visual;
        /*
        self.cursor.doc_cursor_visual = TPos::<u16>{
            cols: u16::from(self.margin_left)+u16::try_from(self.cursor.doc_position.cols).unwrap(),
            rows: u16::try_from(self.cursor.buffer_size.rows).unwrap() - (u16::try_from(self.cursor.doc_offset.rows).unwrap() - u16::try_from(self.cursor.doc_position.rows).unwrap())
        };
        */
    }

    /*
    
    pub(super) fn enter_insert_mode_after(&mut self)  {
        self.mode = EditorMode::Insert;
        self.cursor_type = '6';
        if self.lines[self.cursor.doc_position.rows].len() > self.cursor.doc_position.cols {
            self.cursor.doc_position.cols += 1;
            self.cursor.doc_cursor_visual.cols += 1;
        } 
    }
    
    pub(super) fn enter_insert_mode(&mut self)  {
        self.mode = EditorMode::Insert;
        self.cursor_type = '6';
    }
    
    pub(super) fn delete_char(&mut self) {
        self.lines.delete_char(self.cursor.doc_position).unwrap();
    }
    
    pub(super) fn insert_empty_line_bellow(&mut self) {
        self.lines.insert_line(self.cursor.doc_position.rows+1, "".to_owned()).unwrap();
        
        self.cursor.doc_position.rows += 1; 
        self.cursor.doc_cursor_visual.rows += 1;
        
        self.cursor.doc_position.cols = 0; 
        self.cursor.doc_cursor_visual.cols = u16::from(self.margin_left); 
        
        self.enter_insert_mode();
        self.update_visual_buffer();
    }
    
    
    pub(super) fn insert_empty_line_above(&mut self) {
        self.lines.insert_line(self.cursor.doc_position.rows, "".to_owned()).unwrap();
        
        self.cursor.doc_position.cols = 0; 
        self.cursor.doc_cursor_visual.cols = u16::from(self.margin_left); 
        
        self.enter_insert_mode();
        self.update_visual_buffer();
    }
    
    */
    
}
