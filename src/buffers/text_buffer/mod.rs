mod buffer_logic;
mod graphics;
mod keys;
mod movements;

use super::{
    super::{
        kb,
        h_s::{
            TPos,
            FileMeta,
        },
    },
    GetCursorLocation,
    GetVisualBuffer,
    ProcessKey,
    GetSbData,
    StatusBarData,
};

use std::{
    path::PathBuf,
    cmp,
};

use arrayvec::ArrayString;

#[derive(Default, Debug, Clone, Copy)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
}

enum CursorSelector {
    Main,
    Secondary,
}


impl From<EditorMode> for usize {
    fn from(mode:EditorMode) -> usize {
        match mode {
            EditorMode::Normal => 0,
            EditorMode::Insert => 1,
        }
    }
}

#[allow(dead_code)]
#[derive(Default, Debug)]
enum Numeration {
    Default,
    No,
    Absolute,
    Relative,
    #[default]
    Both,
}

#[derive(Default, Debug)]
pub struct Config {
    numeration: Numeration,
    wrap: bool
    
}

#[derive(Default, Debug)]
pub struct CommandModifiers {
    pub amount:isize,
}


#[derive(Default, Debug)]
pub struct Buffer {
    pub hidden: bool,
    
    pub mode: EditorMode,
    
    pub name: Option<String>,
    pub file_path: Option<PathBuf>,
    
    pub cursor_type: char,
    
    pub margin_left:u8,
    
    pub lines: FileMeta,
    
    pub visual_buffer: String,
    
    pub status_bar_data: Vec<StatusBarData>,
    
    config: Config,
    
    test_cursor: movements::Cursor,
    
    command: CommandModifiers,
}

impl ProcessKey for Buffer {
    fn process_key(&mut self, key:kb::KeyCode) {
        
        if let kb::KeyCode::SpecialKey(kb::SpecialKey::Debug) = key {
            //dbg!(&self.test_cursor);
            panic!("{:?}", self.command);
        }
        match self.mode {
            EditorMode::Normal => {
                self.process_key_visual(key);
            }
            EditorMode::Insert => {
                self.process_key_insert(key);
            }
        }
    }
}

impl GetSbData for Buffer {
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>){
        (&self.status_bar_data[usize::from(self.mode)], self.name.as_ref().map(|x| x.as_str()))
    }
}

impl GetCursorLocation for Buffer {
    fn get_cursor_location(&self) -> (TPos<u16>, char) {
        (self.test_cursor.doc_cursor_visual + self.test_cursor.offset+1, self.cursor_type)
    }
    
}
    
impl GetVisualBuffer for Buffer {
    fn get_visual_buffer(&self) -> &str{
        &self.visual_buffer
    }
}

impl Buffer {
    
    
    pub fn new(offset:TPos<u16>, term_size:TPos<u16>, opening_file:Option<&str>) -> Self {
        
        let doc_offset = TPos::<usize>{
            rows: usize::from(term_size.rows)/2,
            cols: 0//usize::from(term_size.cols)
        };
        
        let doc_cursor_visual_cols = u16::try_from(term_size.rows).unwrap() - (u16::try_from(doc_offset.rows).unwrap() - 0);// u16::try_from(holder.test_cursor.doc_position.rows).unwrap());
        
        
        let lines_holder;
        let file_path;
        let name;
        
        match opening_file {
            None => {
                lines_holder = FileMeta::new();
                file_path = None;
                name = None;
            },
            Some(file) => {
                lines_holder = FileMeta::read_lines(file);
                let file_path_holder = PathBuf::from(file.to_string());
                name = Some(file_path_holder.file_name().unwrap().to_string_lossy().to_string());
                file_path = Some(file_path_holder);
            }
        }
        
        let mut holder = Buffer{
            hidden: false,
            
            name: name,
            file_path: file_path,
            
            lines: lines_holder,
            //doc_cursor_visual: doc_cursor_visual,
            cursor_type: '2',
            
            test_cursor: movements::Cursor::new(offset, term_size, doc_offset),
            ..Self::default()
        };
        holder.update_margin_left();
        holder.update_cursor_location();
        //
        holder.test_cursor.sec_doc_cursor_visual = 0;// TOCHANGE ;
        //
        
        holder.test_cursor.doc_cursor_visual = TPos::<u16>{
            cols: u16::from(holder.margin_left),//u16::try_from(holder.test_cursor.doc_position.cols).unwrap(),
            rows: doc_cursor_visual_cols,
        };
        holder.test_cursor.sec_doc_cursor_visual = holder.test_cursor.doc_cursor_visual.cols;
        
        holder.update_visual_buffer();
        holder.set_status_bar();
        holder
    }
    
    
    fn process_key_visual(&mut self, key:kb::KeyCode) {
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    b'j' => {
                        
                        self.move_cursor_down_visual();
                        self.update_visual_buffer();
                    }
                    b'k' => {
                        self.move_cursor_up_visual();
                        self.update_visual_buffer();
                    }
                    b'l' => {
                        panic!("key 'l'");
                        self.update_visual_buffer();
                        
                    } b'h' => {
                        panic!("key 'h'");
                        self.update_visual_buffer();
                    }
                    b'i' => {
                        self.enter_insert_mode();
                    }
                    b'a' => {
                        self.enter_insert_mode_after();
                    }
                    b'x' => {
                        self.delete_char();
                        self.update_visual_buffer();
                    }
                    b'o' => {
                        self.insert_empty_line_bellow();
                        self.update_visual_buffer();
                    }
                    b'O' => {
                        self.insert_empty_line_above();
                        self.update_visual_buffer();
                    }
                    letter => {
                        panic!("{} not supported yet", letter);
                        
                    }
                }
            }
            kb::KeyCode::Number(number) => {
                self.command.amount *= 10isize;
                self.command.amount += isize::from(number);
            }
            kb::KeyCode::CtrlKey(key) => {
                panic!("ctrl key {}", key as char);
            }
            kb::KeyCode::AltKey(code) => {
                match code {
                    b'w' => {
                        self.write_file();
                    }
                    _ => {
                        panic!("alt key {}", code);
                    }
                }
            }
            kb::KeyCode::AltCtrlKey(code) => {
                panic!("alt ctrl key {}", code);
            }
            kb::KeyCode::SpecialKey(key) => {
                panic!("special key {:?}", key);
            }
            kb::KeyCode::Arrow(arrow) => {
                self.move_doc(arrow);
                self.update_visual_buffer();
            }
        }
    }
    
    

    fn process_key_insert(&mut self, key:kb::KeyCode) {
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    letter => {
                        self.insert_char(char::from(letter));
                    }
                }
            }
            kb::KeyCode::Number(number) => {
                panic!("{}", number);
            }
            kb::KeyCode::CtrlKey(key) => {
                panic!("ctrl key {}", key);
            }
            kb::KeyCode::AltKey(code) => {
                match code {
                    b'a' => {
                        self.mode = EditorMode::Normal;
                        self.cursor_type = '2';
                    }
                    _ => {
                        panic!("alt key {}", code);
                    }
                }
            }
            kb::KeyCode::AltCtrlKey(code) => {
                match code {
                    _ => {
                        panic!("alt ctrl key {}", code);
                    }
                }
            }
            kb::KeyCode::SpecialKey(key) => {
                use kb::SpecialKey::*;
                match key {
                    BackSpace => {
                        self.backspace();
                        self.update_visual_buffer();
                    }
                    Enter => {
                        self.enter_key();
                        self.update_visual_buffer();
                    }
                    Space => {
                        self.insert_char(' ');
                    }
                    Escape => {
                        self.mode = EditorMode::Normal;
                        self.cursor_type = '2';
                    }
                    Tab => {
                        todo!("tab");
                    }
                    Debug => {
                        todo!("should not arrive here ever");
                    }
                }
            }
            kb::KeyCode::Arrow(arrow) => {
                todo!("{:?}", arrow);
                //self.cmd_move_doc(arrow);
            }
        }
    }
    
    

    fn get_column_decoration(&self, deco:&mut String, line:i64, inside_doc:bool) {
        use Numeration::*;
        deco.clear();
        match &self.config.numeration {
            No => {}
            Default => {
                deco.push_str(" ~ ");
            }
            Absolute =>{
                if inside_doc {
                    match usize::try_from(line) {
                        Ok(number) => {
                            let string = number.to_string();
                            for _ in 0..usize::from(self.margin_left)-string.len()-1 {
                                deco.push(' ');
                            }
                            deco.push_str(&string);
                            deco.push(' ');
                        }
                        Err(_) => {
                            for _ in 0..self.margin_left {
                                deco.push('!');
                                
                            }
                            
                        }
                    }
                } else {
                    deco.push('~');
                    for _ in 0..self.margin_left-1 {
                        deco.push(' ');
                    }
                    
                }
            }
            Relative => {
                let line_offset = (i64::try_from(self.test_cursor.doc_position.rows).unwrap() - line).abs();
                if inside_doc {
                    let string = line_offset.to_string();
                    for _ in 0..usize::from(self.margin_left)-string.len()-1 {
                        deco.push(' ');
                    }
                    deco.push_str(&string);
                    deco.push(' ');
                } else {
                    deco.push_str(" ~  ");
                }
                //panic!("{} {}", line_offset);
                
            }
            Both => {
                let line_offset = (i64::try_from(self.test_cursor.doc_position.rows).unwrap() - line).abs();
                
                if line_offset == 0 {
                    let string = line.to_string();
                    deco.push_str(&string);
                    for _ in 0..usize::from(self.margin_left)-string.len() {
                        deco.push(' ');
                    }
                } else if inside_doc {
                    let string = line_offset.to_string();
                    for _ in 0..usize::from(self.margin_left)-string.len()-1 {
                        deco.push(' ');
                    }
                    deco.push_str(&string);
                    deco.push(' ');
                } else {
                    deco.push_str(" ~  ");
                }
                
            }
        }
        
    }
    
    

    fn update_margin_left(&mut self) {
        use Numeration::*;
        let string = self.lines.len().to_string();
        let margin_size = string.len()+1;
        self.margin_left = match self.config.numeration {
            No => 0,
            Default => 3,
            Relative => 4,
            Absolute | Both => cmp::max(margin_size.try_into().unwrap(), 4)
        };
        //panic!("{} {}", self.margin_left, string);
        
    }
    
    

    fn set_status_bar(&mut self) {
        let holder = StatusBarData{
            mode_color: ArrayString::from("\x1b[1;38;5;22;48;5;148m").unwrap(),
            mode_text: ArrayString::from(" NORMAL ").unwrap(),
            file_color: ArrayString::from("\x1b[0;39;48;5;244m").unwrap(),
            middle_color: ArrayString::from("\x1b[0;39;48;5;238m").unwrap(),
        };
        self.status_bar_data.insert(usize::from(EditorMode::Normal), holder);
        
        let holder = StatusBarData{
            mode_color: ArrayString::from("\x1b[1;38;5;196;48;5;208m").unwrap(),
            mode_text: ArrayString::from(" INSERT ").unwrap(),
            file_color: ArrayString::from("\x1b[0;39;48;5;185m").unwrap(),
            middle_color: ArrayString::from("\x1b[0;39;48;5;238m").unwrap(),
        };
        self.status_bar_data.insert(usize::from(EditorMode::Insert), holder);
        
        //panic!("{:#?}", self.status_bar_data);
    }



    fn write_file(&mut self) {
        self.lines.save();
    }

   
}
