#[cfg(test)]
mod tests {
    use algs4_plotting::*;

    #[test]
    fn test_drawing_primitives_no_panic() {
        // Initialize canvas
        set_canvas_size(100, 100).unwrap();
        set_x_scale(0.0, 100.0);
        set_y_scale(0.0, 100.0);

        // Draw some shapes
        set_pen_color(RED);
        line(0.0, 0.0, 100.0, 100.0);

        set_pen_color(BLUE);
        point(50.0, 50.0);

        set_pen_color(GREEN);
        circle(50.0, 50.0, 10.0);

        filled_circle(20.0, 20.0, 5.0);

        rectangle(80.0, 80.0, 10.0, 5.0);
        filled_rectangle(80.0, 20.0, 10.0, 5.0);

        // We cannot easily verify the pixels without exposing the pixmap,
        // but this ensures no panics occur during drawing logic.
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        // Initialize
        set_canvas_size(200, 200).unwrap();

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    // Draw from multiple threads
                    set_pen_color(if i % 2 == 0 { RED } else { BLUE });
                    line(0.0, 0.0, 100.0, 100.0);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
