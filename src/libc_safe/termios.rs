use libc;

use core::{
    fmt,
    fmt::Formatter,
};

#[derive(Clone)]
pub struct TermIOS {
    termios: libc::termios,
}

#[allow(dead_code)]
pub enum TcSetAttrAction {
    TCSANOW,
    TCSADRAIN,
    TCSAFLUSH,
}

pub enum CLFlag{
    ECHO,
    ICANON,
    ISIG,
    IEXTEN
}

pub enum CIFlag{
    IXON,
    ICRNL,
    BRKINT,
    INPCK,
    ISTRIP,
}

pub enum COFlag{
    OPOST,
}

impl TermIOS {
    pub fn new() -> Self {
        Self{
            termios: libc::termios{ c_iflag: 0u32, c_oflag: 0u32, c_cflag: 0u32, c_lflag: 0u32, c_line: 0u8, c_cc: [0u8; 32], c_ispeed: 0u32, c_ospeed: 0u32 },
        }
    }
    
    pub fn tc_get_attr(&mut self, fd:i32) -> Result<(), errno::Errno> {
        if unsafe { libc::tcgetattr(fd, &mut self.termios as *mut libc::termios) } == 0 {
            return Ok(());
        }
        Err(errno::errno())
    }
    
    #[allow(dead_code)]
    pub fn and_on_c_oflag(&mut self, flag:u32) {
        self.termios.c_oflag |= flag;
    }
    
    pub fn turn_off_c_lflag(&mut self, flags:&[CLFlag]) {
        let mut flags_acc = flags.iter().fold(0, |acc, flag| acc | u32::from(flag));
        flags_acc = !flags_acc;
        self.termios.c_lflag &= flags_acc;
    }
    
    pub fn turn_off_c_iflag(&mut self, flags:&[CIFlag]) {
        let mut flags_acc = flags.iter().fold(0, |acc, flag| acc | u32::from(flag));
        flags_acc = !flags_acc;
        self.termios.c_iflag &= flags_acc;
    }
    
    pub fn turn_off_c_oflag(&mut self, flags:&[COFlag]) {
        let mut flags_acc = flags.iter().fold(0, |acc, flag| acc | u32::from(flag));
        flags_acc = !flags_acc;
        self.termios.c_oflag &= flags_acc;
    }
    
    pub fn set_c_cc_time_out(&mut self, minutes:u8, deciseconds:u8) {
        self.termios.c_cc[libc::VMIN] = minutes;
        self.termios.c_cc[libc::VTIME] = deciseconds;
    }
    
    pub fn tc_set_attr(&self, fd:i32, action:TcSetAttrAction) -> Result<(), errno::Errno> {
        if unsafe { libc::tcsetattr(fd, action.into(), &self.termios as *const libc::termios) } == 0 {
            return Ok(());
        }
        Err(errno::errno())
    }
    
}

impl From<&CLFlag> for u32 {
    fn from(action: &CLFlag) -> Self {
        use CLFlag::*;
        match *action {
            ECHO => libc::ECHO,
            ICANON => libc::ICANON,
            ISIG => libc::ISIG,
            IEXTEN => libc::IEXTEN,
        }
    }
}

impl From<&CIFlag> for u32 {
    fn from(action: &CIFlag) -> Self {
        use CIFlag::*;
        match *action {
            IXON => libc::IXON,
            ICRNL => libc::ICRNL,
            BRKINT => libc::BRKINT,
            INPCK => libc::INPCK,
            ISTRIP => libc::ISTRIP,
        }
    }
}

impl From<&COFlag> for u32 {
    fn from(action: &COFlag) -> Self {
        use COFlag::*;
        match *action {
            OPOST => libc::OPOST,
        }
    }
}

impl From<TcSetAttrAction> for i32 {
    fn from(action: TcSetAttrAction) -> Self {
        use TcSetAttrAction::*;
        match action {
            TCSANOW => libc::TCSANOW,
            TCSADRAIN => libc::TCSADRAIN,
            TCSAFLUSH => libc::TCSAFLUSH,
        }
    }
}


impl fmt::Debug for TermIOS {
    fn fmt(&self, formater: &mut Formatter) -> fmt::Result {
        match *self {
            TermIOS{
                ref termios,
            } => {
                termios.fmt(formater)
            }
        }
    }
}
