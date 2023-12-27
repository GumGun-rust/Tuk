use libc;

#[allow(dead_code)]
pub fn is_cntrl(character:i32) -> bool {
    (unsafe{ libc::iscntrl(character) }) == 1
}
