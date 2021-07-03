use ncurses::*;

use crate::ncurses::io::*;
use crate::ncurses::cursor::*;

// default printing functions for ncurses
pub trait Window {
    fn mv_print(&mut self, x: u32, y: u32, text: &str) {
        mvaddstr(y as i32, x as i32, text);
    }

    fn print(&mut self, text: &str) {
        addstr(text);
    }

    fn read_char(&self) -> char {
        getch() as u8 as char
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

impl SubWindow {
    // create the new subwindow and get its information
    pub fn init(x: u32, y: u32, start_x: u32, start_y: u32) -> Self {
        let win = newwin(y as i32,x as i32, start_y as i32, start_x as i32);

        SubWindow { this: win }
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
