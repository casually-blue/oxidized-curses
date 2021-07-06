use crate::cursor::*;
use crate::io_attrs::*;
use crate::window::Window;

pub struct MainWindow {}

impl Window for MainWindow {
    fn move_print(&mut self, point: crate::utils::ScreenPoint, text: &str) {
        ncurses::mvaddstr(point.y as i32, point.x as i32, text);
    }

    fn print(&mut self, text: &str) {
        ncurses::addstr(text);
    }

    fn refresh(&mut self) {
        ncurses::refresh();
    }

    fn get_char(&mut self) -> char {
        ncurses::getch() as u8 as char
    }
}

/// The fullscreen ncurses window
impl MainWindow {
    /// Set up the window with some basic defaults
    pub fn init() -> Self {
        ncurses::initscr();
        set_cbreak(true);
        set_echo(false);
        set_cursor(CursorState::Invisible);
        MainWindow {}
    }
}

impl Drop for MainWindow {
    /// Reset the tty to a reasonable state and clean up curses
    fn drop(&mut self) {
        set_echo(true);
        set_cbreak(false);
        set_cursor(CursorState::Visible);

        ncurses::endwin();
    }
}
