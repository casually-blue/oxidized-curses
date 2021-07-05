use ncurses::CURSOR_VISIBILITY;

pub enum CursorState {
    Visible,
    VeryVisible,
    Invisible,
}

pub fn convert_cursor_visibility(vis: CURSOR_VISIBILITY) -> CursorState {
    use CURSOR_VISIBILITY::*;
    use CursorState::*;
    match vis {
        CURSOR_VISIBLE => Visible,
        CURSOR_INVISIBLE => Invisible,
        CURSOR_VERY_VISIBLE => VeryVisible,
    }
}

pub fn get_curses_curs_visibility(vis: CursorState) -> CURSOR_VISIBILITY {
    use CURSOR_VISIBILITY::*;
    use CursorState::*;

    match vis {
        Visible => CURSOR_VISIBLE,
        Invisible => CURSOR_INVISIBLE,
        VeryVisible => CURSOR_VERY_VISIBLE,
    }
}

pub fn set_cursor(state: CursorState) {
    ncurses::curs_set(get_curses_curs_visibility(state));
}


