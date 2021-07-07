use displaydoc::Display;
use thiserror::Error;
use anyhow::Result;

use crate::window::MainWindow;

#[derive(Debug,Display,Error)]
pub enum ColorError {
    /// This terminal does not support colors
    Unsupported,

    /// Failed to start colors
    StartFailed
}

/// Interface to ncurses color settings
pub struct ColorManager {
}

impl ColorManager {
    pub fn init(_mw: MainWindow) -> Result<ColorManager, ColorError> {
        if !ncurses::has_colors() {
            return Err(ColorError::Unsupported);
        }

        start_color()?;
        Ok(ColorManager { })
    }
}

fn start_color() -> Result<(), ColorError> {
    match ncurses::start_color() {
        x if x > 0 => Ok(()),
        _ => Err(ColorError::StartFailed)
    }
}
