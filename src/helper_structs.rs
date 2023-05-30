#[derive(Debug, Default)]
pub struct TPos<T> {
    pub rows: T,
    pub cols: T,
}

#[derive(Debug)]
pub enum KeyCode {
    Letter(u8),
    Arrow(Arrow),
    //None,
}

#[derive(Debug)]
pub enum Arrow {
    Left,
    Up,
    Right,
    Down,
}

pub enum ScreenMovement {
    Up,
    Down,
}

pub enum CommandMovement {
    EndOfLine,
    StartOfLine,
}

pub enum MoveCommand {
    Arrow(Arrow), 
    Screen(ScreenMovement),
    Command(CommandMovement),
}

/*
#[derive(Clone, Copy)]
pub enum axis {
    x,
    y
}

pub const COLS:axis = axis::x;
pub const ROWS:axis = axis::y;

impl Index<axis> for TPos {
    type Output = u16;
    
    fn index(&self, index:axis) -> &Self::Output {
        use axis::*;
        match index{
            x => &self.x,
            y => &self.y,
        }
    }
}

impl IndexMut<axis> for TPos {
    fn index_mut(&mut self, index:axis) -> &mut Self::Output {
        use axis::*;
        match index{
            x => &mut self.x,
            y => &mut self.y,
        }
    }
}
*/
