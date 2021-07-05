use ncurses::*;

use crate::io_attrs::*;
use crate::cursor::*;
use crate::utils::ScreenRect;

// default printing functions for ncurses
pub trait Window {
    fn mv_print(&mut self, x: u32, y: u32, text: &str) {
        mvaddstr(y as i32, x as i32, text);
    }

    fn print(&mut self, text: &str) {
        addstr(text);
    }
}
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

    pub fn read_char(&self) -> char {
        getch() as u8 as char
    }
}

impl Drop for MainWindow {
    // reset terminal to reasonable state
    // and end ncurses
    fn drop(&mut self) {
        set_echo(true);
        set_cbreak(false);
        set_cursor(CursorState::Visible);

        endwin();
    }
}

pub struct SubWindow {
    // the identifier in ncurses for the window
    this: WINDOW,
}

pub struct SubWindowError {
    pub rect: ScreenRect,
}

impl SubWindow {
    /// Create a new subwindow from the rectangle given
    pub fn init(rect: ScreenRect) -> Result<Self,SubWindowError>{
        let win = newwin(
            rect.offset.y as i32,
            rect.offset.x as i32,
            rect.start.y as i32,
            rect.start.x as i32
        );

        if win == 0 as *mut i8 {
            Err(SubWindowError { rect })
        } else {
            Ok(SubWindow { this: win })
        }
    }
}

impl Window for SubWindow {
    // move the cursor and then print a string to the screen
    fn mv_print(&mut self, x: u32, y: u32, text: &str) {
        mvwaddstr(self.this, y as i32, x as i32, text);
    }

    // print a string to the screen at the current cursor position
    fn print(&mut self, text: &str) {
        waddstr(self.this, text);
    }
}

impl Drop for SubWindow {
    // delete the ncurses window instance
    fn drop(&mut self) {
        delwin(self.this);
    }
}
