use super::{
    //g_libc,
    h_s,
    keyboard,
};

use std::{
    io,
    fs,
    cmp,
};

#[derive(Default, Debug)]
enum BoolConfig {
    #[default]
    No,
    Yes,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
enum Numeration {
    #[default]
    Default,
    No,
    Absolute,
    Relative,
    Both,
}

#[derive(Default, Debug)]
pub struct Config {
    numeration: Numeration,
    wrap: BoolConfig
    
}



#[derive(Default, Debug)]
pub struct Buffer {
    pub hidden: bool,
    pub name: String,
    
    pub offset: h_s::TPos<u16>,
    pub buffer_size: h_s::TPos<u16>,
    
    pub doc_offset: h_s::TPos<usize>,
    
    pub doc_position: h_s::TPos<usize>,
    //position in cursor
    pub doc_cursor_visual: h_s::TPos<u16>,
    //visual position of the cursor relative to the offset
    
    pub margin_left:u8,
    
    pub lines: Vec<String>,
    
    pub visual_buffer: String,
    
    config: Config,
    
}

impl Buffer {
    pub fn new(offset:h_s::TPos<u16>, term_size:h_s::TPos<u16>, opening_file:Option<&str>) -> Self {
        //get to index 0 
        //term_size -= 1;
        
        /*
        offset += 1;
        term_size -= 7;
        */
        
        let doc_offset = h_s::TPos::<usize>{
            //rows: usize::from(term_size.rows)/2,
            rows: usize::from(term_size.rows)-2,
            cols: usize::from(term_size.cols)
        };
        
        /*
        let doc_cursor_visual = h_s::TPos::<u16>{
            rows: term_size.rows/2+1,
            cols: 4+1
        };
        */
        
        let lines_holder = match opening_file {
            None => {
                vec!["".to_owned()]
            },
            Some(file) => {
                Buffer::read_lines(file)
            }
        };
        
        let mut holder = Buffer{
            hidden: false,
            name: "".to_owned(),
            offset: offset,
            buffer_size: term_size,
            doc_offset: doc_offset,
            lines: lines_holder,
            //doc_cursor_visual: doc_cursor_visual,
            
            ..Self::default()
        };
        holder.update_margin_left();
        holder.update_cursor_location();
        holder
    }
    
    pub fn process_key_visual(&mut self, key:keyboard::KeyCode) {
        match key {
            keyboard::KeyCode::Letter(letter) => {
                match letter {
                    b'j' => {
                        self.cmd_move_cursor(keyboard::Arrow::Down);
                        //panic!("r");
                    }
                    b'k' => {
                        self.cmd_move_cursor(keyboard::Arrow::Up);
                        //panic!("k");
                    }
                    _ => {
                        
                    }
                }
            }
            keyboard::KeyCode::Arrow(arrow) => {
                self.cmd_move_doc(arrow);
            }
        }
    }
    
    pub fn process_key_insert(&mut self, key:keyboard::KeyCode) {
        match key {
            keyboard::KeyCode::Letter(letter) => {
                
            }
            keyboard::KeyCode::Arrow(arrow) => {
                //self.cmd_move_doc(arrow);
            }
        }
    }
    
    
    fn cmd_move_doc(&mut self, arrow:keyboard::Arrow) -> Option<()> {
        match arrow {
            keyboard::Arrow::Up => {
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
            keyboard::Arrow::Down => {
                self.doc_offset.rows -= 1;
                self.doc_position.rows -= 1;
                /*
                */
                
            }
            _ => {
                todo!("no side arrows");
            }
        }
        self.update_cursor_location();
        None
    }
    
    fn cmd_move_cursor(&mut self, arrow:keyboard::Arrow) -> Option<()> {
        match arrow {
            keyboard::Arrow::Up => {
                self.doc_position.rows -= 1;
                /*
                panic!("{} {} {} {}", self.buffer_size.rows, self.doc_offset.rows, self.doc_position.rows, self.doc_offset.rows-self.doc_position.rows);
                if usize::from(self.buffer_size.rows) > self.doc_offset.rows-self.doc_position.rows {
                    
                }
                self.doc_position.rows -= 1;
                self.doc_offset.rows -= 1;
                */
                
            }
            keyboard::Arrow::Down => {
                self.doc_position.rows += 1;
                /*
                self.doc_position.rows += 1;
                self.doc_offset.rows += 1;
                */
            }
            _ => {
                todo!("no side arrows");
            }
        }
        self.update_cursor_location();
        None
    }
    
    fn update_cursor_location(&mut self) {
        self.doc_cursor_visual = h_s::TPos::<u16>{
            cols: u16::from(self.margin_left),
            rows: u16::try_from(self.buffer_size.rows).unwrap() - (u16::try_from(self.doc_offset.rows).unwrap() - u16::try_from(self.doc_position.rows).unwrap())
        };
    }
    
    pub fn get_cursor_location(&mut self) -> h_s::TPos<u16> {
        //panic!("{}", self.doc_offset.rows);
        
        self.doc_cursor_visual + self.offset + 1
        /*
        holder += self.offset;
        holder += 1;
        holder
        */
    }
    
    pub fn update_visual_buffer(&mut self) -> &String {
        let mut deco = String::new();
        let pivot_anchor = self.offset+1;
        let next_line = format!("\x1b[{}G\n", pivot_anchor.cols);
        
        self.visual_buffer.clear();
        if self.visual_buffer.len() != 0 {
            panic!("diff de zero");
        }
        self.visual_buffer.push_str(&format!("\x1b[{};{}H", pivot_anchor.rows, pivot_anchor.cols));
        
        for line in (0..=self.buffer_size.rows).rev() {
            /*
            let real_line = i64::try_from(self.doc_position.rows).unwrap()-i64::try_from(line).unwrap();
            let real_line = i64::try_from(self.doc_offset.rows).unwrap()-i64::try_from(line).unwrap()-i64::try_from(self.doc_position.rows).unwrap();
            self.visual_buffer.push_str(DECO);
            self.visual_buffer.push_str(&real_line.to_string());
            self.visual_buffer.push_str("\t\t");
            self.visual_buffer.push_str(&line.to_string());
            self.visual_buffer.push_str("\t\t");
            self.visual_buffer.push_str(&self.doc_position.rows.to_string());
            self.visual_buffer.push_str("\t\t");
            self.visual_buffer.push_str(&self.doc_offset.rows.to_string());
            self.visual_buffer.push_str(&next_line);
            */
            
            let real_line = i64::try_from(self.doc_offset.rows).unwrap()-i64::try_from(line).unwrap()/*-i64::try_from(self.doc_position.rows).unwrap()*/;
            
            let doc_line = match real_line {
                current_line if current_line < 0 => {
                    self.visual_buffer.push_str("\x1b[42m");
                    false
                }
                current_line if usize::try_from(current_line).unwrap() < self.lines.len() => {
                    self.visual_buffer.push_str("\x1b[49m");
                    true
                }
                current_line => {
                    self.visual_buffer.push_str("\x1b[44m");
                    false
                }
            };
            
            /*
            for iter in 0..self.margin_left{
                self.visual_buffer.push('@');
            }
            */
            
            self.get_column_decoration(&mut deco, real_line, doc_line);
            self.visual_buffer.push_str(&deco);
            let data = real_line.to_string();
            self.visual_buffer.push_str(&data);
            for _iter in 0..10-data.len() {
                self.visual_buffer.push_str(" ");
            }
            let data = self.doc_position.rows.to_string();
            self.visual_buffer.push_str(&data);
            for _iter in 0..10-data.len() {
                self.visual_buffer.push_str(" ");
            }
            let data = self.doc_offset.rows.to_string();
            self.visual_buffer.push_str(&data);
            for _iter in 0..10-data.len() {
                self.visual_buffer.push_str(" ");
            }
            self.visual_buffer.push_str(&next_line);
            
        }
        
        &self.visual_buffer
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
    
    fn read_lines(file:&str) -> Vec<String> {
        let file = fs::File::open(file).unwrap();
        let buf_reader = io::BufReader::new(file); 
        let lines = io::BufRead::lines(buf_reader);
        lines.collect::<Result<_, _>>().unwrap()
    }

}

