use ncurses::{endwin, getch, initscr};

use crate::cursor::*;
use crate::io_attrs::*;
use crate::window::Window;

pub struct MainWindow {}

impl Window for MainWindow {}

/// The fullscreen ncurses window
impl MainWindow {
    /// Set up the window with some basic defaults
    pub fn init() -> Self {
        initscr();
        set_cbreak(true);
        set_echo(false);
        set_cursor(CursorState::Invisible);
        MainWindow {}
    }

    /// Get a character from stdin
    pub fn read_char(&self) -> char {
        getch() as u8 as char
    }
}

impl Drop for MainWindow {
    /// Reset the tty to a reasonable state and clean up curses
    fn drop(&mut self) {
        set_echo(true);
        set_cbreak(false);
        set_cursor(CursorState::Visible);

        endwin();
    }
}
