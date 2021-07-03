mod window;
mod cursor;
mod io;

pub use window::*;
pub use cursor::*;
pub use io::*;

use ncurses::*;



pub trait MovableWindow {
    fn movew(&mut self, x: u32, y: u32);
}

pub struct MainWindow {
}

impl Window for MainWindow {
}

impl MainWindow {
    // initialize a reasonable starting
    // state for the main window
    pub fn init() -> Self {
        initscr();
        set_cbreak(true);
        set_echo(false);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        MainWindow {}
    }
}


