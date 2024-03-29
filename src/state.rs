use nix::unistd;
use super::g_libc;
use super::g_libc::TcSetAttrAction;

use std::rc::Rc;
use std::path::PathBuf;

const EXIT_CURSOR:&[u8] = b"\x1b[2 q";

#[derive(Debug)]
pub struct State {
    pub term_fd: Rc<i32>,
    pub regular_exit: bool,
    pub starting_termios: g_libc::TermIOS,
    pub window_state: super::WindowState,
    pub initial_wd: PathBuf,
}

impl Drop for State {
    fn drop(&mut self) {
        self.disable_raw_mode();
        let _ = unistd::write(*self.term_fd, EXIT_CURSOR);
        if self.regular_exit {
            self.window_state.clear_screen();
            let _ = unistd::write(*self.term_fd, &"\x1b[2J".as_bytes());
            let _ = unistd::write(*self.term_fd, &"\x1b[H".as_bytes());
        } else {
            print!("non regular exit\r\n");
        }
    }
}

impl State {
    
    pub fn new(term_fd: i32) -> Self {
        let term_fd = Rc::new(term_fd);
        let initial_wd = std::env::current_exe().unwrap();
        State{
            term_fd: term_fd.clone(),
            starting_termios: g_libc::TermIOS::new(),
            window_state: super::WindowState::new(term_fd),
            initial_wd: initial_wd,
            regular_exit: false, 
        }
    }
    
    pub fn init_editor(&mut self) {
        self.window_state.get_size();
    }

    pub fn enable_raw_mode(&mut self) {
        use g_libc::CLFlag;
        use g_libc::CIFlag;
        use g_libc::COFlag;
        self.starting_termios.tc_get_attr(*self.term_fd).unwrap();
        let mut term = self.starting_termios.clone();
        term.turn_off_c_lflag(&[CLFlag::ECHO, CLFlag::ICANON, CLFlag::ISIG, CLFlag::IEXTEN]);
        term.turn_off_c_iflag(&[CIFlag::IXON, CIFlag::ICRNL, CIFlag::BRKINT, CIFlag::INPCK, CIFlag::ISTRIP]);
        term.turn_off_c_oflag(&[COFlag::OPOST]);
        term.and_on_c_oflag(g_libc::CS8);
        term.set_c_cc_time_out(0u8, 1u8);
        
        term.tc_set_attr(*self.term_fd, TcSetAttrAction::TCSAFLUSH).unwrap();
    }
    
    fn disable_raw_mode(&mut self) {
        self.starting_termios.tc_set_attr(*self.term_fd, TcSetAttrAction::TCSAFLUSH).unwrap();
    }

}
