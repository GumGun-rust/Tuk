use super::{
    GetSbData,
    StatusBarData,
};

#[derive(Debug)]
pub struct Buffer {
    
}


impl GetSbData for Buffer {
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>){
        todo!();
    }
}
