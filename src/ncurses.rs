use ncurses::*;

pub trait Window {
    fn mv_print(&mut self, x: i32, y: i32, text: &str);
    fn print(&mut self, text: &str);
    fn read_char(&self) -> char;
}

pub trait MovableWindow {
    fn movew(&mut self, x: u32, y: u32);
}

pub struct MainWindow {
    this: WINDOW,
}

impl Window for MainWindow {
    fn mv_print(&mut self, x: i32, y: i32, text: &str) {
        mvwaddstr(self.this, y, x, text);
    }

    fn print(&mut self, text: &str) {
        waddstr(self.this, text);
    }

    fn read_char(&self) -> char {
        getch() as u8 as char
    }
}

impl MainWindow {
    pub fn init() -> Self {
        initscr();
        MainWindow {
            this: stdscr(),
        }
    }
}

impl Drop for MainWindow {
    // reset terminal to initial state
    // and end ncurses
    fn drop(&mut self) {
        endwin();
    }
}
