use super::Buffer;

use std::cmp::Ord;

impl Buffer {
    pub(super) fn update_visual_buffer(&mut self) {
        let mut deco = String::new();
        let pivot_anchor = self.cursor.offset+1;
        let next_line = format!("\x1b[{}G\n", pivot_anchor.cols);
        
        self.visual_buffer.clear();
        if self.visual_buffer.len() != 0 {
            panic!("diff de zero");
        }
        self.visual_buffer.push_str(&format!("\x1b[{};{}H", pivot_anchor.rows, pivot_anchor.cols));
        
        for line in (0..=self.cursor.buffer_size.rows).rev() {
            let real_line = i64::try_from(self.cursor.doc_offset.rows).unwrap()-i64::try_from(line).unwrap();//-i64::try_from(self.doc_position.rows).unwrap();
            
            match real_line {
                current_line if current_line < 0 => {
                    self.visual_buffer.push_str("\x1b[42m");
                    self.get_column_decoration(&mut deco, real_line, false);
                    self.visual_buffer.push_str(&deco);
                    for _ in 0..self.cursor.buffer_size.cols-u16::from(self.margin_left)+1 {
                        self.visual_buffer.push(' ');
                    }
                }
                current_line if usize::try_from(current_line).unwrap() < self.lines.len() => {
                    let real_line_usize = usize::try_from(real_line).expect("should only enter when real_line is positivit");
                    
                    self.visual_buffer.push_str("\x1b[49m");
                    self.get_column_decoration(&mut deco, real_line, true);
                    self.visual_buffer.push_str(&deco);
                    
                    
                    if self.cursor.doc_offset.cols < self.lines[real_line_usize].len() {
                        let line_limit = Ord::clamp(self.lines[real_line_usize].len(), 0, usize::from(self.cursor.buffer_size.cols));
                        //let line_limit_u16 = u16::try_from(line_limit).expect("one of the numbers should always fit inside a u16");
                        let printed_chars = u16::try_from(line_limit-self.cursor.doc_offset.cols).expect("should be a name smaller than the buffer");
                        self.visual_buffer.push_str(&self.lines[real_line_usize][self.cursor.doc_offset.cols..line_limit]);
                        
                        self.visual_buffer.push_str("\x1b[45m");
                        
                        for _index in u16::from(self.margin_left)+printed_chars..=self.cursor.buffer_size.cols {
                            self.visual_buffer.push(' ');
                        }
                        self.visual_buffer.push_str("\x1b[0m");
                        
                    } else{
                        
                        self.visual_buffer.push_str("\x1b[46m");
                        for _index in u16::from(self.margin_left)..self.cursor.buffer_size.cols+1 {
                            self.visual_buffer.push(' ');
                        }
                        self.visual_buffer.push_str("\x1b[0m");
                        
                    }
                    
                }
                _current_line => {
                    self.visual_buffer.push_str("\x1b[44m");
                    self.get_column_decoration(&mut deco, real_line, false);
                    self.visual_buffer.push_str(&deco);
                    for _ in 0..self.cursor.buffer_size.cols-u16::from(self.margin_left)+1 {
                        self.visual_buffer.push(' ');
                    }
                }
            }
            
            self.visual_buffer.push_str(&next_line);
            
        }
        
        //self.visual_buffer.push_str(&format!("\x1b[{};{}H", self.cursor.doc_cursor_visual.rows+1, pivot_anchor.cols+u16::from(self.margin_left)+self.sec_doc_cursor_visual));
        //if self.lines[self.doc_position.rows.into()].len() <= self.sec_doc_cursor_visual.into() {
        
        self.visual_buffer.push_str(&format!("\x1b[{};{}H", self.cursor.doc_cursor_visual.rows+1, 0));
        
        
        /*
        */
        if self.cursor.doc_cursor_visual.cols != self.cursor.sec_doc_cursor_visual {
            if self.cursor.doc_cursor_visual.cols <= self.cursor.buffer_size.cols/* TODEL */-10 {
                self.visual_buffer.push_str(&format!("\x1b[{}G\x1b[90;47m paca", pivot_anchor.cols+u16::from(self.margin_left)+self.cursor.sec_doc_cursor_visual));
                //panic!("{} {} {}", self.cursor.doc_cursor_visual.rows+1, 0, pivot_anchor.cols+u16::from(self.margin_left)+self.cursor.sec_doc_cursor_visual);
                
            }
        }
        
        self.visual_buffer.push_str(&format!("\x1b[{}G", pivot_anchor.cols+u16::from(self.margin_left)+self.cursor.sec_doc_cursor_visual));
        
    }

}

