/// A point in curses screen space
pub struct ScreenPoint {
    /// X Coordinate
    pub x: usize,
    /// Y Coordinate
    pub y: usize,
}

/// A rectangle in screen space
pub struct ScreenRect {
    /// The upper left corner of the rectangle
    pub start: ScreenPoint,


    /// The height and width of the rectangle
    pub offset: ScreenPoint,
}
