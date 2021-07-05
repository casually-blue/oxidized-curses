use ncurses::{addstr, mvaddstr};

use crate::utils::ScreenPoint;

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
