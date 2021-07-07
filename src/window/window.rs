use crate::utils::ScreenPoint;

// default printing functions for ncurses
pub trait Window {
    /// Move the cursor to specific coordinates and print a string
    fn move_print(&mut self, point: ScreenPoint, text: &str);

    /// Print a string at the current cursor coordinates
    fn print(&mut self, text: &str);

    fn refresh(&mut self);

    fn get_char(&mut self) -> char;

    fn has_colors() -> bool {
        ncurses::has_colors()
    }

    fn enable_keypad(&mut self, enable: bool);
}
