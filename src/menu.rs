
use json;
use term_size;

struct Window {
    window_height: usize,
    window_width: usize,
    panes: Vec<Pane>,
}

struct Pane {
    x_loc: usize,
    y_loc: usize,
    height: usize,
    width: usize,
}