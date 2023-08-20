use nix::unistd;

use super::{
    h_s::{
        TPos,
    },
    g_libc,
    kb,
    buffer,
    buffers,
};

use std::{
    rc::Rc,
};


#[derive(Default, Debug)]
pub struct WindowState {
    pub term_fd: Rc<i32>,
    pub terminal_size: TPos<u16>,
    pub append_buffer: String,
    pub current_buff: usize,
    pub main_buffers: Vec<buffer::Buffer>,
    pub main_buffers_v2: Vec<buffers::Buffers>,
    pub status_bar: bool,
}

impl WindowState {
    
    
    pub fn new(term_fd: Rc<i32>) -> Self {
        let buffer_vec_holder:Vec<buffer::Buffer> = Vec::new();
        let holder = Self{
            term_fd: term_fd, 
            main_buffers: buffer_vec_holder,
            .. WindowState::default() 
        };
        holder
    }
    
    

    pub fn start_editor(&mut self, opening_file:Option<&str>) {
        let buffer_size = TPos::<u16>{
            rows: self.terminal_size.rows-2,
            cols: self.terminal_size.cols,//-30,
            //..self.terminal_size
        };
        let offset = TPos::<u16>{
            rows: 0,
            cols: 0,
        };
        
        let buffer_holder = buffer::Buffer::new(
            offset,
            buffer_size-1,
            opening_file,
        );
        
        let buffer_holder_v2 = buffers::Buffers::new_text(
            offset,
            buffer_size-1,
            opening_file,
        );
        
        self.main_buffers.push(buffer_holder);
        self.main_buffers_v2.push(buffer_holder_v2);
        
        let _ = unistd::write(*self.term_fd, b"\x1b[2J");
        /* clear screen */
        
        
    }
    

    
    pub fn process_key(&mut self) -> Option<()>{
        match self.read_key() {
            Some(read_key) => {
                self.main_buffers[self.current_buff].process_key(read_key);
            },
            None => {}
        }
        
        Some(())
        //println!("\r\n{:?}", read_key);
    }

    

    pub fn read_key(&self) -> Option<kb::KeyCode>/*Option<i64>*/ {
        let mut buffer = [0u8];
        match unistd::read(*self.term_fd, &mut buffer[..]) {
            Ok(value) => {
                if value == 0 {
                    return None
                } 
                match buffer[0] {
                    b'q' => {
                        panic!("q to abort");
                    }
                    b'\x1b' => Some(self.handle_esc_code().into()),
                    letter @ _ =>  Some(kb::KeyCode::Letter(letter)),
                }
            },
            Err(err) => {
                println!("{:?}", err);
                panic!("err");
            }
        }
    }
    
    

    fn handle_esc_code(&self) -> kb::EscapeCode {
        let mut buffer = [0u8;3];
        match unistd::read(*self.term_fd, &mut buffer[..]) {
            Ok(value) => {
                if value == 0 {
                    return kb::EscapeCode::Esc;
                }
                if buffer[0] != b'[' {
                    return kb::EscapeCode::Esc;
                }
                match buffer[1] {
                    65 => kb::EscapeCode::ArrowUp,
                    66 => kb::EscapeCode::ArrowDown,
                    67 => kb::EscapeCode::ArrowRight,
                    68 => kb::EscapeCode::ArrowLeft,
                    number @ _ => {
                        todo!(" escape code not supported yet buffer Code {}", number);
                    }
                }
            },
            Err(err) => {
                panic!("esc code {}", err);
                //println!("{:?}", err);
            }
        }
        
    }

    

    pub fn get_size(&mut self) {
        let mut win_size = g_libc::WinSize::new();
        if let Ok(_) = win_size.io_ctl(*self.term_fd, g_libc::WinSizeRequest::TIOCGWINSZ) {
            win_size.get_window_size(&mut self.terminal_size.rows, &mut self.terminal_size.cols);
            //self.margins_left = 3;
            //self.margins_bottom = 2;
            return;
        }
        panic!("ioctl Not Supported");
    }
    
    

    pub fn clear_screen(&mut self) {
        panic!();
    }

    

    pub fn render_screen(&mut self) {
        
        self.append_buffer.clear();
        
        let updated_buffer = self.main_buffers[self.current_buff].update_visual_buffer();
        
        self.append_buffer.push_str(updated_buffer);
        
        let cursor_location = self.main_buffers[self.current_buff].get_cursor_location();
        //let file_name = self.main_buffers[self.current_buff].get_buffer_name().clone();
        
        self.status_bar();
        
        self.append_buffer.push_str(&format!("\x1b[{};{}H", cursor_location.rows, cursor_location.cols));
        let _ = unistd::write(*self.term_fd, &self.append_buffer.as_bytes());
        
        return;
    }
    
    

    fn status_bar(&mut self) {
        self.append_buffer.push_str(&format!("\x1b[{:?};0H", self.terminal_size.rows-1));
        
        let (status_line_data, file_name) = self.main_buffers[self.current_buff].get_status_bar_data();
        let file_name = match file_name {
            Some(file) => file,
            None => ""
        };
        
        
        self.append_buffer.push_str(&format!("{}{}{} {} {}", 
                status_line_data.mode_color, 
                status_line_data.mode_text,
                status_line_data.file_color, 
                file_name,
                status_line_data.middle_color));
        
        for _columns in 2..usize::try_from(self.terminal_size.cols).unwrap()-status_line_data.mode_text.len()-file_name.len() {
            self.append_buffer.push(' ');
        }
        self.append_buffer.push_str("\x1b[49m");
        
        
    }
    
    

}

