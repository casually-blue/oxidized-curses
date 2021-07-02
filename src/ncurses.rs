use ncurses::*;

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
    initial_cursor: CURSOR_VISIBILITY,
    initial_echo: bool,
    initial_cbreak: bool,
}

impl Window for MainWindow {
}

impl MainWindow {
    pub fn init() -> Self {
        initscr();
        set_cbreak(false);
        set_echo(false);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        MainWindow {
            initial_cursor: CURSOR_VISIBILITY::CURSOR_VISIBLE,
            initial_echo: true,
            initial_cbreak: true,
        }
    }
}

fn set_echo(echo_val: bool){
    if echo_val {
        echo();
    } else {
        noecho();
    }
}

fn set_cbreak(cbreak_val: bool){
    if cbreak_val {
        cbreak();
    } else {
        nocbreak();
    }
}

impl Drop for MainWindow {
    // reset terminal to initial state
    // and end ncurses
    fn drop(&mut self) {
        set_echo(self.initial_echo);
        set_cbreak(self.initial_cbreak);
        curs_set(self.initial_cursor);

        endwin();
    }
}

pub struct SubWindow {
    // the identifier in ncurses for the window
    this: WINDOW,
}

impl SubWindow {
    pub fn init(x: u32, y: u32, start_x: u32, start_y: u32) -> Self {
        let win = newwin(y as i32,x as i32, start_y as i32, start_x as i32);

        SubWindow { this: win }
    }
}

impl Window for SubWindow {
    fn mv_print(&mut self, x: u32, y: u32, text: &str) {
        mvwaddstr(self.this, y as i32, x as i32, text);
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
