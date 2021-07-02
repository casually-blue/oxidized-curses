use ncurses::*;

pub trait Window {
    fn create(height: u32, width: u32, startx: u32, starty: u32);
    fn move_win(&mut self, x: u32, y: u32);
}

pub struct MainWindow {
    this: WINDOW,
}

impl MainWindow {
    pub fn init() -> Self {
        initscr();
        MainWindow {
            this: stdscr(),
        }
    }

    // print a string to the screen at a specific location
    // note: position parameters are backwards from ncurses
    pub fn mvprint(&mut self, x: i32, y: i32, text: &str) {
       mvwaddstr(self.this, y, x, text);
    }

    pub fn print(&mut self, text: &str) {
        waddstr(self.this, text);
    }

    pub fn read_char(&self) -> char {
        getch() as u8 as char
    }
}

impl Drop for MainWindow {
    // reset terminal to initial state
    // and end ncurses
    fn drop(&mut self) {
        endwin();
    }
}
