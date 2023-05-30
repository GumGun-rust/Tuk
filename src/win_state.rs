use super::helper_structs;
use nix::unistd;
use std::rc::Rc;
use super::libc_safe as g_libc;

use std::{
    io,
    fs::File,
};


#[derive(Default, Debug)]
pub struct WindowState {
    pub term_fd: Rc<i32>,
    
    pub offset: helper_structs::TPos<u16>,
    pub buffer_size: helper_structs::TPos<u16>,
    
    pub doc_start: helper_structs::TPos<usize>,
    pub cursor_doc: helper_structs::TPos<usize>,
    
    pub margins_left:u8,
    pub margins_bottom:u8,
    
    pub append_buffer: String,
    pub lines: Vec<String>,
    
    config: Config,
}

#[derive(Default, Debug)]
struct Config {
    scroll_off: u8,
}

impl WindowState {
    
    pub fn new(term_fd: Rc<i32>) -> Self {
        Self{
            term_fd: term_fd, 
            cursor_doc: helper_structs::TPos::<usize>{cols:1, rows:1}, 
            doc_start: helper_structs::TPos::<usize>{cols:0, rows:0}, 
            .. WindowState::default() 
        }
    }
    
    pub fn start_editor(&mut self, opening_file:Option<&str>) {
        self.config.scroll_off = 3;
        match opening_file {
            None => {
                println!("no_file\r");
            },
            Some(file) => {
                println!("opening\r\n{:?}\r", file);
                self.lines = read_lines(file);
                println!("{:?}", &self.lines);
            }
        }
    }
    
    pub fn process_key(&mut self) -> Option<()>{
        const K_KEYPRESS:u8 = b'k' as u8;
        const J_KEYPRESS:u8 = b'j' as u8;
        const H_KEYPRESS:u8 = b'h' as u8;
        const L_KEYPRESS:u8 = b'l' as u8;
        const M_KEYPRESS:u8 = b'm' as u8;
        const COMA_KEYPRESS:u8 = b',' as u8;
        const DOLLAR_SIGN_KEYPRESS:u8 = b'$' as u8;
        const NUMBER_0_KEYPRESS:u8 = b'0' as u8;
        
        match self.read_key() {
            helper_structs::KeyCode::Letter(data) => {
                match data {
                    b'q' => {
                        return None //panic!("quitt {} {}", value, b'0' as u8 as u32);
                    },
                    K_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Arrow(helper_structs::Arrow::Up));
                    },
                    J_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Arrow(helper_structs::Arrow::Down));
                    },
                    H_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Arrow(helper_structs::Arrow::Left));
                    },
                    L_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Arrow(helper_structs::Arrow::Right));
                    },
                    M_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Screen(helper_structs::ScreenMovement::Down));
                    }
                    COMA_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Screen(helper_structs::ScreenMovement::Up));
                    }
                    DOLLAR_SIGN_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Command(helper_structs::CommandMovement::EndOfLine));
                    }
                    NUMBER_0_KEYPRESS => {
                        self.move_cursor(helper_structs::MoveCommand::Command(helper_structs::CommandMovement::StartOfLine));
                    }
                    _ => {
                        
                        if g_libc::is_cntrl(data.try_into().unwrap()) {
                            print!("{}\r\n", data);
                        } else {
                            print!("{} {}\r\n", data, char::try_from(u32::try_from(data).unwrap()).unwrap());
                        }
                    }
                }
                return Some(())
            },
            helper_structs::KeyCode::Arrow(direction) => {
                self.move_cursor(helper_structs::MoveCommand::Arrow(direction));
                return Some(())
            }
        }
    }


    pub fn move_cursor(&mut self, action:helper_structs::MoveCommand) {
        match action {
            helper_structs::MoveCommand::Arrow(direction) => {
                use helper_structs::Arrow::*;
                match direction {
                    Left => {
                        if self.cursor_doc.cols > 0 && self.cursor_doc.cols > 0 {
                            self.cursor_doc.cols -=1;
                        }
                    },
                    Up => {
                        if self.cursor_doc.rows > 1 {
                            self.cursor_doc.rows -= 1;
                        }
                    },
                    Right => {
                        if self.cursor_doc.cols < (self.buffer_size.cols-u16::from(self.margins_left)).into() {
                            if self.lines[self.cursor_doc.rows+self.doc_start.rows-1].len() > usize::from(self.cursor_doc.cols) {
                                self.cursor_doc.cols += 1;
                            }
                        }
                    },
                    Down => {
                        if self.doc_start.rows+self.cursor_doc.rows < self.lines.len() && usize::from(self.buffer_size.rows-u16::from(self.margins_bottom)) > self.cursor_doc.rows {
                            self.cursor_doc.rows += 1;
                        } 
                    },
                }
            },
            helper_structs::MoveCommand::Screen(direction) => {
                use helper_structs::ScreenMovement::*;
                match direction {
                    Up => {
                        if self.doc_start.rows > 0 {
                            self.doc_start.rows -= 1;
                        }
                        else {
                            panic!("nothing to scroll Up");
                        }
                    },
                    Down => {
                        if self.doc_start.rows+1 < self.lines.len() {
                            self.doc_start.rows += 1;
                            if self.doc_start.rows+self.cursor_doc.rows > self.lines.len() {
                                self.cursor_doc.rows -= 1;
                            }
                        } 
                        else {
                            panic!("nothing to scroll Down");
                        }
                    },
                }
            },
            helper_structs::MoveCommand::Command(command) => {
                use helper_structs::CommandMovement::*;
                match command {
                    EndOfLine => {
                        self.cursor_doc.cols = self.lines[self.cursor_doc.rows+self.doc_start.rows-1].len();
                    },
                    StartOfLine => {
                        self.cursor_doc.cols = 0;
                    },
                }
            },
        }
    }
    
    pub fn read_key(&mut self) -> helper_structs::KeyCode/*Option<i64>*/ {
        use helper_structs::KeyCode::*;
        use helper_structs::Arrow;
        let mut buffer = [0u8];
        let mut esc_buffer = [0u8, 0u8];
        loop {
            match unistd::read(*self.term_fd, &mut buffer[..]) {
                Ok(data) => {
                    if buffer[0] == b'\x1b' {
                        let mut read_status = unistd::read(*self.term_fd, &mut esc_buffer[0..1]);
                        if let Err(err) = read_status {
                            panic!("{} {}", err, errno::errno());
                        }
                        if Ok(1) != read_status {
                            return Letter(buffer[0]);
                        }
                        read_status = unistd::read(*self.term_fd, &mut esc_buffer[1..2]);
                        if let Err(err) = read_status {
                            panic!("{} {}", err, errno::errno());
                        }
                        if Ok(1) != read_status {
                            return Letter(buffer[0]);
                        }
                        if esc_buffer[0] != b'[' {
                            return Letter(buffer[0]);
                        }
                        return match esc_buffer[1] {
                            b'C' => Arrow(Arrow::Right),
                            b'A' => Arrow(Arrow::Up),
                            b'B' => Arrow(Arrow::Down),
                            b'D' => Arrow(Arrow::Left),
                            _ => {panic!("escape code not supported yet lol");},
                        };
                    } else {
                        if data == 0 {
                            continue;
                        }
                        return Letter(buffer[0]);
                    }
                },
                Err(err) => {
                    panic!("{} {}", err, errno::errno());
                },
            }
        }
    }

    
    pub fn get_size(&mut self) {
        let mut win_size = g_libc::WinSize::new();
        if let Ok(_) = win_size.io_ctl(*self.term_fd, g_libc::WinSizeRequest::TIOCGWINSZ) {
            win_size.get_window_size(&mut self.buffer_size.rows, &mut self.buffer_size.cols);
            self.margins_left = 3;
            self.margins_bottom = 2;
            return;
        }
        panic!("ioctl Not Supported");
    }
    
    pub fn clear_screen(&mut self) {
        let _ = unistd::write(*self.term_fd, &"\x1b[2J".as_bytes());
        let _ = unistd::write(*self.term_fd, &"\x1b[H".as_bytes());
    }

    pub fn render_screen(&mut self) {
        
        let buffer_rows = self.buffer_size.rows-u16::from(self.margins_bottom);
        
        self.append_buffer.clear();
        
        self.append_buffer.push_str("\x1b[0;0H");
        
        for line in 0..buffer_rows {
            self.append_buffer.push_str("\x1b[K");
            self.append_buffer.push_str(" ~ ");
            
            let line_index = usize::from(line)+self.doc_start.rows;
            if self.lines.len() > line_index {
                
                self.append_buffer.push_str(&self.lines[usize::from(line_index)]);
                /*
                let lines_to_print = if self.lines[usize::from(line_index)].len() <= self.buffer_size.rows.into() {
                    &self.lines[usize::from(line_index)]
                } else {
                    &self.lines[usize::from(line_index)][0..self.buffer_size.rows.into()]
                };
                self.append_buffer.push_str(lines_to_print);
                */
                
            }
            
            if line != buffer_rows-1 {
                self.append_buffer.push_str("\r\n");
            }
        } 
        
        self.append_buffer.push_str("\x1b[42m\r\n");
        for _ in 0..self.buffer_size.cols {
            self.append_buffer.push(' ');
        }
        self.append_buffer.push_str("\x1b[49m");
        /*
        */
        
        //self.append_buffer.push_str("\r\n\x1b[42m Normal \x1b[47m                                                \x1b[49m");
        
        let cursor_column = if self.cursor_doc.cols == 0 {
            usize::from(self.margins_left)+1
        } else {
            usize::from(self.margins_left)+self.cursor_doc.cols
        };
        self.append_buffer.push_str(&format!("\x1b[{};{}H", self.cursor_doc.rows, cursor_column));
        
        let _ = unistd::write(*self.term_fd, &self.append_buffer.as_bytes());
    }
}

fn read_lines(file:&str) -> Vec<String> {
    let file = File::open(file).unwrap();
    let buf_reader = io::BufReader::new(file); 
    let lines = io::BufRead::lines(buf_reader);
    lines.collect::<Result<_, _>>().unwrap()
}

