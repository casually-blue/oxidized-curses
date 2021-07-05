use crate::utils::ScreenPoint;

/// A window that isn't fixed in screen space
pub trait MoveableWindow {
    fn move_window(&mut self, point: ScreenPoint);
}
