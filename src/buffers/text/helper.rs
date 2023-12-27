use std::default::Default;
use std::mem::take;

#[derive(Debug)]
pub struct CommandModifiers {
    pub amount:usize,
}

impl Default for CommandModifiers {
    fn default() -> Self {
        CommandModifiers{
            amount:3,
        }
    }
}


impl CommandModifiers {
    pub fn take(&mut self) -> CommandModifiers {
        take(self)
    }
}
