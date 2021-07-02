use ncurses::*;

pub trait Window {
    fn mv_print(&mut self, x: i32, y: i32, text: &str) {
        mvaddstr(y, x, text);
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
    pub fn init() -> Self {
        initscr();
        MainWindow { }
    }
}

impl Drop for MainWindow {
    // reset terminal to initial state
    // and end ncurses
    fn drop(&mut self) {
        endwin();
    }
}

pub struct SubWindow {
    this: WINDOW,
}

impl SubWindow {
    pub fn init(x: i32, y: i32, start_x: i32, start_y: i32) -> Self {
        let win = newwin(y,x, start_y, start_x);

        SubWindow { this: win }
    }
}

impl Window for SubWindow {
    fn mv_print(&mut self, x: i32, y: i32, text: &str) {
        mvwaddstr(self.this, y, x, text);
    }

    fn print(&mut self, text: &str) {
        waddstr(self.this, text);
    }
}

impl Drop for SubWindow {
    fn drop(&mut self) {
        delwin(self.this);
    }
}
