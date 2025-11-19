use algs4_plotting::*;

fn main() {
    // Set canvas size
    set_canvas_size(800, 400).unwrap();

    // Set coordinate scale
    set_x_scale(0.0, std::f64::consts::PI * 4.0);
    set_y_scale(-1.2, 1.2);

    // Draw axes
    set_pen_radius(0.005);
    set_pen_color(BLACK);
    line(0.0, 0.0, std::f64::consts::PI * 4.0, 0.0); // X axis
    line(0.0, -1.2, 0.0, 1.2); // Y axis

    // Plot sine wave
    set_pen_radius(0.01);
    set_pen_color(BLUE);

    let mut x = 0.0;
    let dx = 0.1;
    while x < std::f64::consts::PI * 4.0 {
        line(x, x.sin(), x + dx, (x + dx).sin());
        x += dx;
    }

    // Plot cosine wave
    set_pen_color(RED);
    x = 0.0; // Reuse x
    while x < std::f64::consts::PI * 4.0 {
        point(x, x.cos());
        x += dx;
    }

    // Show and wait
    println!("Press Escape to exit.");
    while is_open() && !is_key_down(Key::Escape) {
        show(100);
    }
}
