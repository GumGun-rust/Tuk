use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
};


#[derive(Debug, Default)]
pub struct TPos<T> {
    pub rows: T,
    pub cols: T,
}

impl<T> TPos<T> {
    pub fn new(rows:T, cols:T) -> Self {
        Self{
            rows,
            cols
        }
    }
}

impl<T:Clone> Clone for TPos<T>{
    fn clone(&self) -> Self {
        Self{
            rows: self.rows.clone(),
            cols: self.cols.clone(),
        }
    }
}

impl<T:Copy> Copy for TPos<T> {}

/*
pub enum MoveCommand {
    //Arrow(Arrow), 
    Screen(ScreenMovement),
    Command(CommandMovement),
}

pub enum ScreenMovement {
    Up,
    Down,
}
pub enum CommandMovement {
    EndOfLine,
    StartOfLine,
}
*/

impl<T:Add<Output = T>> Add<Self> for TPos<T> {
    type Output = TPos<T>;
    fn add(self, other:Self) -> Self::Output {
        TPos{
            cols:self.cols+other.cols,
            rows:self.rows+other.rows
        }
    }
}

impl<T:Add<Output = T>+Clone> Add<T> for TPos<T> {
    type Output = TPos<T>;
    fn add(self, value:T) -> Self::Output {
        TPos{
            cols:self.cols+value.clone(),
            rows:self.rows+value.clone()
        }
    }
}

impl<T:Add<Output = T>+Clone> Add<&Self> for TPos<T> {
    type Output = TPos<T>;
    fn add(self, other:&Self) -> Self::Output {
        TPos{
            cols:self.cols+other.cols.clone(),
            rows:self.rows+other.rows.clone()
        }
    }
}

impl<T:Add<Output = T>+Clone> Add<&T> for TPos<T> {
    type Output = TPos<T>;
    fn add(self, value:&T) -> Self::Output {
        TPos{
            cols:self.cols+value.clone(),
            rows:self.rows+value.clone()
        }
    }
}

impl<T:AddAssign> AddAssign<Self> for TPos<T> {
    fn add_assign(&mut self, other:Self) {
        self.cols += other.cols;
        self.rows += other.rows;
    }
}

impl<T:AddAssign+Clone> AddAssign<T> for TPos<T> {
    fn add_assign(&mut self, other:T) {
        self.cols += other.clone();
        self.rows += other;
    }
}

impl<T:AddAssign+Clone> AddAssign<&Self> for TPos<T> {
    fn add_assign(&mut self, other:&Self) {
        self.cols += other.cols.clone();
        self.rows += other.rows.clone();
    }
}

impl<T:AddAssign+Clone> AddAssign<&T> for TPos<T> {
    fn add_assign(&mut self, other:&T) {
        self.cols += other.clone();
        self.rows += other.clone();
    }
}

impl<T:SubAssign> SubAssign<Self> for TPos<T> {
    fn sub_assign(&mut self, other:Self) {
        self.cols -= other.cols;
        self.rows -= other.rows;
    }
}

impl<T:SubAssign+Clone> SubAssign<T> for TPos<T> {
    fn sub_assign(&mut self, other:T) {
        self.cols -= other.clone();
        self.rows -= other;
    }
}

impl<T:SubAssign+Clone> SubAssign<&Self> for TPos<T> {
    fn sub_assign(&mut self, other:&Self) {
        self.cols -= other.cols.clone();
        self.rows -= other.rows.clone();
    }
}

impl<T:SubAssign+Clone> SubAssign<&T> for TPos<T> {
    fn sub_assign(&mut self, other:&T) {
        self.cols -= other.clone();
        self.rows -= other.clone();
    }
}

impl<T:Sub<Output = T>+Clone> Sub<T> for TPos<T> {
    type Output = TPos<T>;
    fn sub(self, value:T) -> Self::Output {
        TPos{
            cols:self.cols-value.clone(),
            rows:self.rows-value.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t_pos_traits() {
        let mut test_tpos0 = TPos::<usize>{ cols: 0, rows: 1, };
        let test_tpos1 = TPos::<usize>{ cols: 1, rows: 0, };
        dbg!(&test_tpos0);
        dbg!(&test_tpos1);
        test_tpos0 = test_tpos0 + &test_tpos1;
        dbg!(&test_tpos0);
        test_tpos0 += &test_tpos1;
        dbg!(&test_tpos0);
        test_tpos0 -= &test_tpos1;
        dbg!(&test_tpos0);
    }
}
