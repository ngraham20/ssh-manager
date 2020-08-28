
use json;

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