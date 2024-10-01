use super::Buffer;
use super::StatusBarData;
use super::GetSbData;

use super::EditorMode;

use arrayvec::ArrayString;


/*
TODO: Change it to a constant array for the diferent styles this change is dependant on the const creation of the ArrayVec's type ArrayString which looks like it may be dead,
check one day
*/

impl GetSbData for Buffer {
    fn get_sb_data(&self) -> (&StatusBarData, Option<&str>){
        (&self.status_bar_data[usize::from(self.mode)], self.name.as_ref().map(|x| x.as_str()))
    }
}

impl Buffer {
    
    pub(super)
    fn set_status_bar(&mut self) {
        let holder = StatusBarData{
            mode_color: ArrayString::from("\x1b[1;38;5;22;48;5;148m").unwrap(),
            mode_text: ArrayString::from(" NORMAL ").unwrap(),
            file_color: ArrayString::from("\x1b[0;39;48;5;244m").unwrap(),
            middle_color: ArrayString::from("\x1b[0;39;48;5;238m").unwrap(),
        };
        self.status_bar_data.insert(usize::from(EditorMode::normal_default()), holder);
        
        let holder = StatusBarData{
            mode_color: ArrayString::from("\x1b[1;38;5;196;48;5;208m").unwrap(),
            mode_text: ArrayString::from(" INSERT ").unwrap(),
            file_color: ArrayString::from("\x1b[0;39;48;5;185m").unwrap(),
            middle_color: ArrayString::from("\x1b[0;39;48;5;238m").unwrap(),
        };
        self.status_bar_data.insert(usize::from(EditorMode::Insert), holder);
    }

}
