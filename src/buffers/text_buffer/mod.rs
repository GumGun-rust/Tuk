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
pub struct Buffer {
    pub hidden: bool,
    
    pub mode: EditorMode,
    
    pub name: Option<String>,
    pub file_path: Option<PathBuf>,
    
    pub offset: TPos<u16>,
    pub buffer_size: TPos<u16>,
    
    pub doc_offset: TPos<usize>,
    
    //position in cursor
    pub doc_position: TPos<usize>,
    
    //visual position of the cursor relative to the offset
    pub doc_cursor_visual: TPos<u16>,
    pub cursor_type: char,
    
    pub margin_left:u8,
    
    pub lines: FileMeta,
    
    pub visual_buffer: String,
    
    pub status_bar_data: Vec<StatusBarData>,
    
    config: Config,
    
}

impl ProcessKey for Buffer {
    fn process_key(&mut self, key:kb::KeyCode) {
        
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
        (self.doc_cursor_visual + self.offset + 1, self.cursor_type)
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
            cols: usize::from(term_size.cols)
        };
        
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
            offset: offset,
            
            buffer_size: term_size,
            doc_offset: doc_offset,
            lines: lines_holder,
            //doc_cursor_visual: doc_cursor_visual,
            cursor_type: '2',
            
            ..Self::default()
        };
        holder.update_margin_left();
        holder.update_cursor_location();
        holder.update_visual_buffer();
        holder.set_status_bar();
        holder
    }
    
    
    fn process_key_visual(&mut self, key:kb::KeyCode) {
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    b'j' => {
                        self.cmd_move_cursor(kb::Arrow::Down);
                        self.update_visual_buffer();
                    }
                    b'k' => {
                        self.cmd_move_cursor(kb::Arrow::Up);
                        self.update_visual_buffer();
                    }
                    b'l' => {
                        self.cmd_move_cursor(kb::Arrow::Right);
                        self.update_visual_buffer();
                        
                    } b'h' => {
                        self.cmd_move_cursor(kb::Arrow::Left);
                        self.update_visual_buffer();
                    }
                    b'i' => {
                        self.mode = EditorMode::Insert;
                        self.cursor_type = '6';
                    }
                    b'a' => {
                        self.mode = EditorMode::Insert;
                        self.cursor_type = '6';
                        if self.lines[self.doc_position.rows].len() > self.doc_position.cols {
                            self.doc_position.cols += 1;
                            self.doc_cursor_visual.cols += 1;
                        } 
                    }
                    b'x' => {
                        self.lines.delete_char(self.doc_position).unwrap();
                        self.update_visual_buffer();
                    }
                    letter => {
                        panic!("{} not supported yet", letter);
                        
                    }
                }
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
                self.cmd_move_doc(arrow);
                self.update_visual_buffer();
            }
        }
    }
    
    

    fn process_key_insert(&mut self, key:kb::KeyCode) {
        match key {
            kb::KeyCode::Letter(letter) => {
                match letter {
                    letter => {
                        self.insert_char_logic(char::from(letter));
                    }
                }
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
                        if self.doc_position.cols > 0 {
                            self.doc_position.cols -= 1; 
                            self.lines.delete_char(self.doc_position).unwrap();
                            self.doc_cursor_visual.cols -= 1; 
                        } else {
                            if self.doc_position.rows > 0 {
                                self.doc_position.rows -= 1; 
                                self.doc_cursor_visual.rows -= 1;
                                self.doc_position.cols = self.lines.get_line_len(self.doc_position.rows); 
                                
                                self.doc_cursor_visual.cols = u16::from(self.margin_left)+u16::try_from(self.lines.get_line_len(self.doc_position.rows)).expect("long line not supported"); 
                                
                                self.lines.fuse_lines(self.doc_position.rows).unwrap();
                            }
                        }
                        self.update_visual_buffer();
                    }
                    Enter => {
                        let holder = self.lines[self.doc_position.rows][self.doc_position.cols..].to_string();
                        self.lines[self.doc_position.rows].replace_range(self.doc_position.cols.., "");
                        self.lines.insert_line(self.doc_position.rows+1, holder).unwrap();
                        self.doc_position.rows += 1; 
                        self.doc_position.cols = 0; 
                        self.doc_cursor_visual.cols = u16::from(self.margin_left); 
                        self.doc_cursor_visual.rows += 1;
                        self.update_visual_buffer();
                    }
                    Space => {
                        self.insert_char_logic(' ');
                    }
                    Escape => {
                        self.mode = EditorMode::Normal;
                        self.cursor_type = '2';
                    }
                    Tab => {
                        todo!("tab");
                    }
                }
            }
            kb::KeyCode::Arrow(arrow) => {
                todo!("{:?}", arrow);
                //self.cmd_move_doc(arrow);
            }
        }
    }
    
    
    #[inline(always)]
    fn insert_char_logic(&mut self, letter:char) {
        self.lines.insert_char(self.doc_position, letter).unwrap(); 
        self.doc_position.cols += 1; 
        self.doc_cursor_visual.cols += 1; 
        self.update_visual_buffer();
        
    }
    
    
    fn cmd_move_doc(&mut self, arrow:kb::Arrow) -> Option<()> {
        match arrow {
            kb::Arrow::Up => {
                self.doc_offset.rows += 1;
                self.doc_position.rows += 1;
                /*
                let number = i64::try_from(self.doc_offset.rows).unwrap()-i64::try_from(self.lines.len()).unwrap();
                if number+1 < self.buffer_size.rows.into() {
                    self.doc_offset.rows += 1;
                    //self.doc_cursor_visual.rows -= 1;
                }
                */
            }
            kb::Arrow::Down => {
                self.doc_offset.rows -= 1;
                self.doc_position.rows -= 1;
                /*
                */
                
            }
            /*
            kb::Arrow::Left => {
            }
            kb::Arrow::Right => {
            }
            */
            _ => {
                todo!("no side arrows");
            }
        }
        self.update_cursor_location();
        None
    }
    
    
    
    fn cmd_move_cursor(&mut self, arrow:kb::Arrow) -> Option<()> {
        use kb::Arrow::*;
        match arrow {
            Up => {
                self.doc_position.rows -= 1;
                /*
                panic!("{} {} {} {}", self.buffer_size.rows, self.doc_offset.rows, self.doc_position.rows, self.doc_offset.rows-self.doc_position.rows);
                if usize::from(self.buffer_size.rows) > self.doc_offset.rows-self.doc_position.rows {
                    
                }
                self.doc_position.rows -= 1;
                self.doc_offset.rows -= 1;
                */
                
            }
            Down => {
                self.doc_position.rows += 1;
                /*
                self.doc_position.rows += 1;
                self.doc_offset.rows += 1;
                */
            }
            Left => {
                if self.doc_position.cols != 0 {
                    self.doc_position.cols -= 1;
                }
                //todo!("no side arrow left");
            }
            Right => {
                if self.doc_position.cols <= self.lines[self.doc_position.rows].len()-1 {
                    self.doc_position.cols += 1;
                }
                //todo!("no side arrows right");
            }
        }
        self.update_cursor_location();
        None
    }
    

    
    fn update_cursor_location(&mut self) {
        self.doc_cursor_visual = TPos::<u16>{
            cols: u16::from(self.margin_left)+u16::try_from(self.doc_position.cols).unwrap(),
            rows: u16::try_from(self.buffer_size.rows).unwrap() - (u16::try_from(self.doc_offset.rows).unwrap() - u16::try_from(self.doc_position.rows).unwrap())
        };
    }
    
    
    
    fn update_visual_buffer(&mut self) {
        let mut deco = String::new();
        let pivot_anchor = self.offset+1;
        let next_line = format!("\x1b[{}G\n", pivot_anchor.cols);
        
        self.visual_buffer.clear();
        if self.visual_buffer.len() != 0 {
            panic!("diff de zero");
        }
        self.visual_buffer.push_str(&format!("\x1b[{};{}H", pivot_anchor.rows, pivot_anchor.cols));
        
        for line in (0..=self.buffer_size.rows).rev() {
            let real_line = i64::try_from(self.doc_offset.rows).unwrap()-i64::try_from(line).unwrap();//-i64::try_from(self.doc_position.rows).unwrap();
            
            match real_line {
                current_line if current_line < 0 => {
                    self.visual_buffer.push_str("\x1b[42m");
                    self.get_column_decoration(&mut deco, real_line, false);
                    self.visual_buffer.push_str(&deco);
                    for _ in 0..self.buffer_size.cols-u16::from(self.margin_left)+1 {
                        self.visual_buffer.push(' ');
                    }
                }
                current_line if usize::try_from(current_line).unwrap() < self.lines.len() => {
                    self.visual_buffer.push_str("\x1b[49m");
                    self.get_column_decoration(&mut deco, real_line, true);
                    self.visual_buffer.push_str(&deco);
                    
                    self.visual_buffer.push_str(&self.lines[usize::try_from(real_line).unwrap()]);
                    
                    for _ in 0..usize::from(self.buffer_size.cols)-usize::from(self.margin_left)+1-self.lines[usize::try_from(real_line).unwrap()].len() {
                        self.visual_buffer.push(' ');
                    }
                    
                }
                _current_line => {
                    self.visual_buffer.push_str("\x1b[44m");
                    self.get_column_decoration(&mut deco, real_line, false);
                    self.visual_buffer.push_str(&deco);
                    for _ in 0..self.buffer_size.cols-u16::from(self.margin_left)+1 {
                        self.visual_buffer.push(' ');
                    }
                }
            }
            
            self.visual_buffer.push_str(&next_line);
            
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
                let line_offset = (i64::try_from(self.doc_position.rows).unwrap() - line).abs();
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
                let line_offset = (i64::try_from(self.doc_position.rows).unwrap() - line).abs();
                
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


