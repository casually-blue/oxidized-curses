use ncurses::CURSOR_VISIBILITY;

pub enum CursorState {
    Visible,
    VeryVisible,
    Invisible,
}

fn get_curses_curs_visibility(vis: CursorState) -> CURSOR_VISIBILITY {
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


