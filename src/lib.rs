/// Cursor visibility functions
pub mod cursor;

/// IO settings functions
pub mod io_attrs;

/// Internal Data Structures
pub mod utils;

/// Window management code
pub mod window;

/// Color Management Code
pub mod color;

#[cfg(test)]
mod tests {
    #[test]
    fn init_curses(){
        let w = crate::window::MainWindow::init();
    }
}
