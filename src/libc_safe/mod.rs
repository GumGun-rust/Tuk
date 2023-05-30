use libc;

pub use libc::{
    STDIN_FILENO,
    CS8,
};

mod termios;
pub use termios::{
    TermIOS,
    TcSetAttrAction,
    CLFlag,
    CIFlag,
    COFlag,
};

mod win_size;
pub use win_size::{
    WinSize,
    WinSizeRequest,
};
    

mod misc;
pub use misc::{
    is_cntrl,
};
