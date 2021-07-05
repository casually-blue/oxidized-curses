use ncurses::*;

use crate::io_attrs::*;
use crate::cursor::*;
use crate::utils::{ScreenRect,ScreenPoint};

// default printing functions for ncurses
pub trait Window {
    /// Move the cursor to specific coordinates and print a string
    fn move_print(&mut self, point: ScreenPoint, text: &str) {
        mvaddstr(point.y as i32, point.x as i32, text);
    }

    /// Print a string at the current cursor coordinates
    fn print(&mut self, text: &str) {
        addstr(text);
    }
}

/// A window that isn't fixed in screen space
pub trait MovableWindow {
    fn movew(&mut self, x: u32, y: u32);
}

pub struct MainWindow {
}

impl Window for MainWindow {
}

/// The fullscreen ncurses window
impl MainWindow {
    /// Set up the window with some basic defaults
    pub fn init() -> Self {
        initscr();
        set_cbreak(true);
        set_echo(false);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
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

/// A window inside the main window
pub struct SubWindow {
    // the identifier in ncurses for the window
    this: WINDOW,
}

/// Subwindow Creation Error
pub enum SubWindowError {
    CoordinateError(ScreenRect),
    OtherError,
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
            Err(SubWindowError::CoordinateError(rect))
        } else {
            Ok(SubWindow { this: win })
        }
    }
}

impl Window for SubWindow {
    /// Move the cursor and print a string at specific coordinates on the subwindow
    fn move_print(&mut self, point: ScreenPoint, text: &str) {
        mvwaddstr(self.this, point.y as i32, point.x as i32, text);
    }

    /// Print a string at the curent subwindow cursor position
    fn print(&mut self, text: &str) {
        waddstr(self.this, text);
    }
}

impl Drop for SubWindow {
    /// Free the internal subwindow pointer
    fn drop(&mut self) {
        delwin(self.this);
    }
}
