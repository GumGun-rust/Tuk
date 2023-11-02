use super::{
    TPos,
};

use std::{
    ops::{
        Index,
        IndexMut,
    },
    io::{
        self,
        Write,
        Seek,
    },
    fs::{
        self,
        File,
    },
};


#[derive(Debug, Default)]
pub struct FileMeta {
    pub lines: Vec<String>,
    pub char_count: usize,
    pub file: Option<File>,
}


impl FileMeta {
    pub fn new() -> Self {
        Self{
            lines: vec!["".to_owned()],
            char_count: 0,
            file: None,
        }
    }
    
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    
    pub fn read_lines(file:&str) -> Self {
        let file = fs::File::options().write(true).read(true).open(file).unwrap();
        let len:usize = file.metadata().unwrap().len().try_into().unwrap();
        let mut buf_reader = io::BufReader::new(file); 
        let lines = io::BufRead::lines(&mut buf_reader);
        let lines:Vec<_> = lines.collect::<Result<_, _>>().unwrap();
        
        let file = buf_reader.into_inner();
        
        /*
        let letter_count = lines.iter().fold(0,|acc, line|{
            acc + line.len()
        });
        if letter_count+lines.len() != len {
            panic!("son diferentes {} {}", letter_count, len);
        }
        */
        
        Self{
            lines: lines,
            char_count: len,
            file: Some(file),
        }
    }

    pub fn insert_line(&mut self, location:usize, holder:String) -> Result<(),()> {
        self.lines.insert(location, holder);
        Ok(())
    }
    
    pub fn insert_char(&mut self, location:TPos<usize>, caracter:char) -> Result<(),()> {
        self.lines[location.rows].insert(location.cols, caracter);
        Ok(())
    }
    
    pub fn delete_line(&mut self, location:usize) -> Result<(), ()> {
        self.lines.remove(location);
        Ok(())
    }
    
    pub fn delete_char(&mut self, location:TPos<usize>) -> Result<(),()> {
        self.lines[location.rows].remove(location.cols);
        Ok(())
    }
    
    #[inline(always)]
    pub fn fuse_lines(&mut self, location:usize) -> Result<(), ()> {
        //self.lines[location].pop();
        let holder = self.lines.remove(location+1);
        
        self.lines[location].push_str(&holder);
        Ok(())
    }
    
    pub fn save(&mut self) {
        match &mut self.file {
            Some(file) => {
                file.set_len(0).expect("restarting whole file");
                file.rewind().expect("rewinding file");
                let mut buf_writer = io::BufWriter::with_capacity(self.char_count, file);
                for (index, line) in self.lines.iter().enumerate() {
                    buf_writer.write(line.as_bytes()).unwrap();
                    if index != self.lines.len()-1 {
                        buf_writer.write(b"\n").unwrap();
                    }
                }
                buf_writer.flush().unwrap();
            }
            None => {
                todo!("logic when file does not exists");
            }
            
        }
        
    }
    
    
    pub fn get_line_len(&self, index:usize) -> usize {
        self.lines[index].len()
    }
    
    
}


impl Index<usize> for FileMeta {
    type Output = String;
    
    fn index(&self, index:usize) -> &Self::Output {
        &self.lines[index]
    }
}

impl IndexMut<usize> for FileMeta {
    fn index_mut(&mut self, index:usize) -> &mut Self::Output {
        &mut self.lines[index]
    }
    
}

