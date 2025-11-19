use algs4_plotting::*;

fn main() {
    set_canvas_size(600, 600).unwrap();
    set_x_scale(-1.0, 1.0);
    set_y_scale(-1.0, 1.0);

    let mut rx: f64 = 0.480;
    let mut ry: f64 = 0.860;
    let mut vx: f64 = 0.015;
    let mut vy: f64 = 0.023;
    let radius: f64 = 0.05;

    println!("Press Escape to exit.");

    // Enable double buffering for smooth animation
    enable_double_buffering();

    while is_open() && !is_key_down(Key::Escape) {
        // Update position
        if rx.abs() + radius > 1.0 {
            vx = -vx;
        }
        if ry.abs() + radius > 1.0 {
            vy = -vy;
        }
        rx += vx;
        ry += vy;

        // Clear background
        clear(WHITE); // White

        // Draw ball
        set_pen_color(BLACK);
        filled_circle(rx, ry, radius);

        // Show and wait
        show(20);
    }
}
