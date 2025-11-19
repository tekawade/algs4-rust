pub mod std_draw;

// Re-export commonly used functions for easier access
pub use std_draw::{
    circle,
    clear,
    disable_double_buffering,

    enable_double_buffering,
    filled_circle,
    filled_rectangle,
    line,
    // Drawing primitives
    point,
    rectangle,
    // Control
    set_canvas_size,
    set_pen_color,
    set_pen_radius,
    set_x_scale,
    set_y_scale,
    show,
    text,

    // Constants
    BLACK,
    BLUE,
    CYAN,
    GREEN,
    MAGENTA,
    RED,
    WHITE,
    YELLOW,
};
