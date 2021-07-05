use ncurses::*;

// tell ncurses whether to echo typed
// characters to the screen
pub fn set_echo(echo_val: bool) {
    if echo_val {
        echo();
    } else {
        noecho();
    }
}

// tell ncurses whether to get raw input or not
pub fn set_raw(raw_val: bool) {
    if raw_val {
        raw();
    } else {
        noraw();
    }
}

// set whether ncurses gets raw input or not
// minus control characters
pub fn set_cbreak(cbreak_val: bool) {
    if cbreak_val {
        cbreak();
    } else {
        nocbreak();
    }
}
