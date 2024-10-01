
use super::Buffer;

pub fn move_up(buffer:&mut Buffer, modifier:usize) {
    let holder = buffer.cursor.calculate_up(modifier);
    buffer.cursor.apply(holder);
    buffer.update_visual_buffer();
}

pub fn move_down(buffer:&mut Buffer, buffer_size:usize, modifier:usize) {
    let holder = buffer.cursor.calculate_down(buffer_size, modifier);
    buffer.cursor.apply(holder);
    buffer.update_visual_buffer();
}
