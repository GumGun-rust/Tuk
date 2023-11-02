/*
use nix::unistd;
use errno;
*/
use std::env;

mod libc_safe;
use libc_safe as g_libc;

mod helper_structs;
use helper_structs as h_s;

mod keyboard;
use keyboard as kb;

mod state;
use state::State;

mod win_state;
use win_state::WindowState;

mod buffers;

use g_libc::{
    STDIN_FILENO,
};

fn main() {
    let mut state = State::new(STDIN_FILENO);
    //let mut og_termios = &mut state.starting_termios;
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    state.enable_raw_mode();
    state.init_editor();

    let w_s = &mut state.window_state;
    
    let file = if args.len() > 1 {
        Some(args[1].as_ref()) 
    } else {
        None
    };
    
    w_s.start_editor(file);
    
    //state.render_screen(state.window_state.rows);
    loop {
        match w_s.process_key() {
            None => {
                break;
            },
            Some(_) => {
                //WindowText::clear_screen();
                w_s.render_screen();
            }
        }
        
    }
    state.regular_exit = true;
}
