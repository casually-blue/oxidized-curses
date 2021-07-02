use ncurses::*;

pub struct MainWindow {

}

impl MainWindow {
    pub fn init() -> Self {
        initscr();
        MainWindow {}
    }

    pub fn print(&self, x: i32, y: i32, text: &str) {
       mvaddstr(y, x, text);
    }

    pub fn read_char(&self) -> char {
        getch() as u8 as char
    }
}

impl Drop for MainWindow {
    fn drop(&mut self) {
        endwin();
    }
}
