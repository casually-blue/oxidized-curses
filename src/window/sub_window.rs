use crate::utils::{ScreenPoint, ScreenRect};
use crate::window::{MoveableWindow, Window};

/// A window inside the main window
pub struct SubWindow {
    // the identifier in ncurses for the window
    this: ncurses::WINDOW,
}

/// Subwindow Creation Error
pub enum SubWindowError {
    CoordinateError(ScreenRect),
    OtherError,
}

impl SubWindow {
    /// Create a new subwindow from the rectangle given
    pub fn init(rect: ScreenRect) -> Result<Self, SubWindowError> {
        let win = ncurses::newwin(
            rect.offset.y as i32,
            rect.offset.x as i32,
            rect.start.y as i32,
            rect.start.x as i32,
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
        ncurses::mvwaddstr(self.this, point.y as i32, point.x as i32, text);
    }

    /// Print a string at the curent subwindow cursor position
    fn print(&mut self, text: &str) {
        ncurses::waddstr(self.this, text);
    }

    fn refresh(&mut self) {
        ncurses::wrefresh(self.this);
    }

    fn get_char(&mut self) -> char {
        ncurses::wgetch(self.this) as u8 as char
    }

    fn enable_keypad(&mut self, enable: bool) {
        ncurses::keypad(self.this, enable);
    }
}

impl MoveableWindow for SubWindow {
    fn move_window(&mut self, point: ScreenPoint) {
        ncurses::mvwin(self.this, point.y as i32, point.x as i32);
    }
}

impl Drop for SubWindow {
    /// Free the internal subwindow pointer
    fn drop(&mut self) {
        ncurses::delwin(self.this);
    }
}
