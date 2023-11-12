use super::{
    movements::{
        BackspaceAction,
    },
    Buffer
};

impl Buffer {
    pub(super) fn backspace(&mut self) {
        
        match self.test_cursor.backspace_movement(self.margin_left, self.lines.get_line_len(self.test_cursor.doc_position.rows)) {
            BackspaceAction::DeleteChar => {
                self.lines.delete_char(self.test_cursor.doc_position).unwrap();
            }
            BackspaceAction::FuseLine => {
                self.lines.fuse_lines(self.test_cursor.doc_position.rows).unwrap();
            }
            BackspaceAction::None => { }
        }
    }
    
    pub(super) fn enter(&mut self) {
        let holder = self.lines[self.test_cursor.doc_position.rows][self.test_cursor.doc_position.cols..].to_string();
        self.lines[self.test_cursor.doc_position.rows].replace_range(self.test_cursor.doc_position.cols.., "");
        self.lines.insert_line(self.test_cursor.doc_position.rows+1, holder).unwrap();
        self.test_cursor.enter_movement(self.margin_left);
    }
    
}
    
