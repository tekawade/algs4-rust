pub mod std_draw;

// Re-export Key for input handling
pub use minifb::Key;

// Re-export commonly used functions for easier access
pub use std_draw::{
    // Drawing primitives
    circle,
    clear,
    // Control
    disable_double_buffering,
    enable_double_buffering,
    filled_circle,
    filled_rectangle,
    is_key_down,
    is_open,
    line,
    point,
    rectangle,
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
