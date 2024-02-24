mod buffer_logic;
mod status_bar;
mod graphics;
mod keys;
mod movements;
mod helper;
mod substates;

use movements::Cursor;
use substates::InputState;

use super::super::kb;
use super::super::h_s::TPos;
use super::super::h_s::FileMeta;
use super::super::h_s::CursorType;
use super::GetCursorLocation;
use super::GetVisualBuffer;
use super::ProcessKey;
use super::GetSbData;
use super::StatusBarData;
use std::path::PathBuf;
use std::cmp::max;
use debug_ignore::DebugIgnore;



#[derive(Default, Debug, Clone, Copy)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
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

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Config {
    cursor_type: CursorType,
    numeration: Numeration,
    wrap: bool,
    
    //amount of margin line on move operations
    pub scrolloff: u16,
    
}


#[derive(Default, Debug)]
pub struct Buffer {
    pub visual_buffer: DebugIgnore<String>,
    pub status_bar_data: DebugIgnore<Vec<StatusBarData>>,
    
    pub hidden: bool,
    
    pub mode: EditorMode,
    
    pub name: Option<String>,
    
    pub file_path: Option<PathBuf>,
    
    pub margin_left:u8,
    
    pub lines: FileMeta,
    
    config: Config,
    
    cursor: Cursor,
    
    state: InputState,
}

impl ProcessKey for Buffer {
    fn process_key(&mut self, key:kb::KeyCode) {
        
        if let kb::KeyCode::SpecialKey(kb::SpecialKey::Debug) = key {
            panic!("debug key {:#?}", self);
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

impl GetCursorLocation for Buffer {
    fn get_cursor_location(&self) -> (TPos<u16>, char) {
        (self.cursor.doc_cursor_visual + self.cursor.offset+1, char::from(self.config.cursor_type))
    }
    
}
    
impl GetVisualBuffer for Buffer {
    fn get_visual_buffer(&self) -> &str{
        &self.visual_buffer
    }
}

impl Buffer {
    
    
    pub fn new(
        offset:TPos<u16>, 
        term_size:TPos<u16>, 
        opening_file:Option<&str>
    ) -> Self {
        
        let doc_offset = TPos::<usize>{
            rows: usize::from(term_size.rows)/2,
            cols: 0//usize::from(term_size.cols)
        };
        
        let doc_cursor_visual_cols = u16::try_from(term_size.rows).unwrap() - (u16::try_from(doc_offset.rows).unwrap() - 0);// u16::try_from(holder.cursor.doc_position.rows).unwrap());
        
        
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
            
            config: Config{
                scrolloff: 2,
                cursor_type: CursorType::Block2,
                ..Config::default()
            },
            
            //modifier: CommandModifiers::default(),
            
            cursor: Cursor::new(offset, term_size, doc_offset),
            ..Self::default()
        };
        holder.update_margin_left();
        holder.update_cursor_location();
        
        //
        holder.cursor.sec_doc_cursor_visual = 0;// TOCHANGE ;
        //
        
        holder.cursor.doc_cursor_visual = TPos::<u16>{
            cols: u16::from(holder.margin_left),//u16::try_from(holder.cursor.doc_position.cols).unwrap(),
            rows: doc_cursor_visual_cols,
        };
        holder.cursor.sec_doc_cursor_visual = holder.cursor.doc_cursor_visual.cols;
        
        holder.update_visual_buffer();
        holder.set_status_bar();
        holder
    }
    
    
    fn process_key_visual(&mut self, key:kb::KeyCode) {
        let key = match self.current_substate().apply(self, key) {
            Some(key) => key,
            None => {return;}
        };
        
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    b'd' => {
                        eprintln!("{:?}", letter);
                        eprintln!("{:#?}", self);
                        eprintln!("{:?}", self.lines.len())
                    }
                    b'j' => {  
                        self.key_j();
                    } 
                    b'J' => {  
                        self.key_J();
                    } 
                    b'k' => {  
                        self.key_k();
                    } 
                    b'K' => {  
                        self.key_K();
                    } 
                    b'z' => {
                        self.key_z();
                    }
                    b'Z' => {
                        self.key_Z();
                    }
                    _ => {}
                    
                } 
                
            }
            kb::KeyCode::Number(number) => {
                self.key_number(number);
            }
            _ => {panic!();}
        }
    }
    
    #[allow(dead_code)]
    fn process_key_insert(&mut self, _key:kb::KeyCode) {
        
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
                let line_offset = (i64::try_from(self.cursor.doc_position.rows).unwrap() - line).abs();
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
                let line_offset = (i64::try_from(self.cursor.doc_position.rows).unwrap() - line).abs();
                
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
            Absolute | Both => max(margin_size.try_into().unwrap(), 4)
        };
        //panic!("{} {}", self.margin_left, string);
    }

    #[allow(dead_code)]
    fn write_file(&mut self) {
        self.lines.save();
    }
    
    pub fn size(&self) -> usize {
        self.lines.len()
    }
}

impl From<EditorMode> for usize {
    fn from(mode:EditorMode) -> usize {
        match mode {
            EditorMode::Normal => 0,
            EditorMode::Insert => 1,
        }
    }
}

