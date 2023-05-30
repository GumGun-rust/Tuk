use nix::unistd;
use super::{
    g_libc,
    g_libc::{
        TcSetAttrAction,
    },
};
use std::rc::Rc;

#[derive(Debug)]
pub struct State {
    pub term_fd: Rc<i32>,
    pub regular_exit: bool,
    pub starting_termios: g_libc::TermIOS,
    pub window_state: super::WindowState,
}

impl Drop for State{
    fn drop(&mut self) {
        self.disable_raw_mode();
        if self.regular_exit {
            self.window_state.clear_screen();
            let _ = unistd::write(*self.term_fd, &"\x1b[2J".as_bytes());
            let _ = unistd::write(*self.term_fd, &"\x1b[H".as_bytes());
            //dbg!(&self.window_state.raw_terminal_size);
            dbg!(&self.window_state.buffer_size);
            dbg!(&self.window_state.doc_start);
            dbg!(&self.window_state.cursor_doc);
            //print!("hola desde drop\r\nrows: {}\r\ncols: {}\r\n", self.window_state.raw_terminal_size.rows, self.window_state.raw_terminal_size.cols);
        } else {
            print!("non regular exit\r\n");
        }
    }
}

impl State {
    
    pub fn new(term_fd: i32) -> Self {
        let term_fd = Rc::new(term_fd);
        State{
            term_fd: term_fd.clone(),
            starting_termios: g_libc::TermIOS::new(),
            window_state: super::WindowState::new(term_fd),
            regular_exit: false, 
        }
    }
    
    pub fn start_editor(&mut self) {
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
