use nix::unistd;

use super::h_s::TPos;
use super::g_libc;
use super::kb;
use super::buffers;
use super::buffers::GetCursorLocation;
use super::buffers::ProcessKey;
use super::buffers::GetVisualBuffer;
use super::buffers::GetSbData;
use super::buffers::MoveWindow;

use std::rc::Rc;


#[derive(Default, Debug, Clone, Copy)]
pub enum Mode{
    #[default]
    Other,
    Command,
}

#[derive(Debug)]
pub enum KeyEvent{
    Normal(kb::KeyCode),
    Command,
    Esc,
    None,
}

#[derive(Default, Debug)]
pub struct WindowState {
    pub term_fd: Rc<i32>,
    pub terminal_size: TPos<u16>,
    pub append_buffer: String,
    pub current_buff: usize,
    pub main_buffers_v2: Vec<buffers::Buffers>,
    pub status_bar: bool,
    
    pub mode: Mode,
    pub command: String,
}

impl WindowState {
    
    
    pub fn new(term_fd: Rc<i32>) -> Self {
        let holder = Self{
            term_fd: term_fd, 
            .. WindowState::default() 
        };
        holder
    }
    
    

    pub fn start_editor(&mut self, opening_file:Option<&str>) {
        let buffer_size = TPos::<u16>{
            rows: self.terminal_size.rows-20,
            cols: self.terminal_size.cols-30,
            //..self.terminal_size
        };
        let offset = TPos::<u16>{
            rows: 10,
            cols: 10,
        };
        
        let buffer_holder_v2 = buffers::Buffers::new_text(
            offset,
            buffer_size-1,
            opening_file,
        );
        
        self.main_buffers_v2.push(buffer_holder_v2);
        
        let _ = unistd::write(*self.term_fd, b"\x1b[2J");
        /* clear screen */
        
        
    }
    

    
    pub fn process_key(&mut self) -> Option<()>{
        match self.read_key() {
            KeyEvent::Normal(read_key) => {
                self.main_buffers_v2[self.current_buff].move_window(TPos::default());
                self.main_buffers_v2[self.current_buff].process_key(read_key);
            },
            KeyEvent::Command => {
                self.mode = Mode::Command;
            }
            KeyEvent::Esc => {
                match self.mode {
                    Mode::Command => {
                        self.mode = Mode::Other;
                    }
                    Mode::Other => {
                        self.main_buffers_v2[self.current_buff].process_key(kb::EscapeCode::Esc.into());
                    }
                    
                }
            }
            KeyEvent::None => {}
        }
        
        Some(())
        //println!("\r\n{:?}", read_key);
    }

    

    pub fn read_key(&self) -> KeyEvent {
        
        const BACKSPACE:u8 = b'';
        const TAB:u8 = b'\t';
        const ENTER:u8 = 13;
        const SPACE:u8 = 32;
        
        let mut buffer = [0u8];
        match unistd::read(*self.term_fd, &mut buffer[..]) {
            Ok(value) => {
                if value == 0 {
                    return KeyEvent::None
                } 
                match buffer[0] {
                    b'q' => {todo!("q to abort");}
                    
                    b':' => KeyEvent::Command,
                    
                    b'Q' => KeyEvent::Normal(kb::KeyCode::SpecialKey(kb::SpecialKey::Debug)),
                    
                    b'\x1b' => {
                        let potential_escape_code = self.handle_esc_code().into();
                        match potential_escape_code {
                            kb::EscapeCode::Esc => KeyEvent::Esc,
                            escape_code @ _ => KeyEvent::Normal(escape_code.into()),
                        }
                        
                    }
                    
                    letter @ b'a'..=b'z' => KeyEvent::Normal(kb::KeyCode::Letter(letter)),
                    
                    letter @ b'A'..=b'Z' => KeyEvent::Normal(kb::KeyCode::Letter(letter)),
                    
                    BACKSPACE => KeyEvent::Normal(kb::KeyCode::SpecialKey(kb::SpecialKey::BackSpace)),
                    
                    TAB => KeyEvent::Normal(kb::KeyCode::SpecialKey(kb::SpecialKey::Tab)),
                    
                    ENTER => KeyEvent::Normal(kb::KeyCode::SpecialKey(kb::SpecialKey::Enter)),
                    
                    SPACE => KeyEvent::Normal(kb::KeyCode::SpecialKey(kb::SpecialKey::Space)),
                    
                    letter @ 1..=26 => KeyEvent::Normal(kb::KeyCode::CtrlKey(letter+b'a'-1)),
                    
                    number @ b'0'..=b'9' => KeyEvent::Normal(kb::KeyCode::Number(number-b'0')),
                    
                    letter => {todo!("other key {}", letter);}
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
                match value{
                    0 => {
                        return kb::EscapeCode::Esc;
                    }
                    1 => {
                        match buffer[0] {
                            letter @ b'a'..=b'z' => kb::EscapeCode::AltKey(letter), 
                            letter @ 1..=26 => kb::EscapeCode::AltCtrlKey(letter+b'a'-1),
                            letter => {
                                todo!("unsuported letter {}", letter);
                            }
                        }
                    }
                    2 => {
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
                    }
                    value => {
                        todo!("implementing {} {:?}", value, buffer);
                    }
                }
            },
            Err(err) => {
                panic!("esc code {}", err);
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
        
        let updated_buffer = self.main_buffers_v2[self.current_buff].get_visual_buffer();
        
        self.append_buffer.push_str(updated_buffer);
        
        let (cursor_location, cursor_type) = self.main_buffers_v2[self.current_buff].get_cursor_location();
        //let file_name = self.main_buffers[self.current_buff].get_buffer_name().clone();
        
        self.build_status_bar();
        
        
        match self.mode {
            Mode::Other => {
                self.clear_command_line();
                self.append_buffer.push_str(&format!("\x1b[{} q\x1b[{};{}H", cursor_type, cursor_location.rows, cursor_location.cols));
            }
            Mode::Command => {
                self.append_buffer.push_str(&format!("\x1b[0 q\x1b[{};0H:", self.terminal_size.rows));
            }
        }
        
        let _ = unistd::write(*self.term_fd, &self.append_buffer.as_bytes());
        //panic!("{}", self.append_buffer);
        
    }
    
    

    fn build_status_bar(&mut self) {
        
        self.append_buffer.push_str(&format!("\x1b[{:?};0H", self.terminal_size.rows-1));
        
        let (status_line_data, file_name) = self.main_buffers_v2[self.current_buff].get_sb_data();
        let file_name = match file_name {
            Some(file) => file,
            None => ""
        };
        
        let text_holder = match self.mode {
            Mode::Other => &status_line_data.mode_text,
            Mode::Command => "  CMD   ",
        };
        
        self.append_buffer.push_str(&format!("{}{}{} {} {}", 
            status_line_data.mode_color, 
            text_holder,
            //status_line_data.mode_text,
            status_line_data.file_color, 
            file_name,
            status_line_data.middle_color
        ));
        
        
        for _columns in 2..usize::try_from(self.terminal_size.cols).unwrap()-status_line_data.mode_text.len()-file_name.len() {
            self.append_buffer.push(' ');
        }
        self.append_buffer.push_str("\x1b[49m");
        
        
    }
    
    
    fn clear_command_line(&mut self) {
        self.append_buffer.push_str(" \x1b[2K");
    }

}

