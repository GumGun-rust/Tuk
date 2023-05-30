use libc;

use core::{
    fmt,
    fmt::Formatter,
};

pub struct WinSize {
    winsize: libc::winsize,
}

pub enum WinSizeRequest {
    TIOCGWINSZ
}

impl WinSize {
    pub fn new() -> Self {
        Self{ winsize: libc::winsize{ ws_row: 0u16, ws_col: 0u16, ws_xpixel: 0u16, ws_ypixel: 0u16 }, }
    }
    
    pub fn io_ctl(&mut self, fd:i32, request:WinSizeRequest) -> Result<(), errno::Errno> {
        if unsafe { libc::ioctl(fd, u64::from(request), &mut self.winsize as *mut libc::winsize) } == 0 {
            return Ok(());
        }
        Err(errno::errno())
    }
    
    pub fn get_window_size(&self, rows:&mut u16, cols:&mut u16) {
        *rows = self.winsize.ws_row;
        *cols = self.winsize.ws_col;
    }
}

impl From<WinSizeRequest> for u64 {
    fn from(action: WinSizeRequest) -> Self {
        use WinSizeRequest::*;
        match action {
            TIOCGWINSZ => libc::TIOCGWINSZ,
        }
    }
}

impl fmt::Debug for WinSize {
    fn fmt(&self, formater: &mut Formatter) -> fmt::Result {
        match *self {
            WinSize{
                ref winsize,
            } => {
                winsize.fmt(formater)
            }
        }
    }
}
