use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;
use std::sync::{LazyLock, Mutex};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

// Default settings
const DEFAULT_SIZE: usize = 512;
const DEFAULT_PEN_RADIUS: f32 = 0.002; // Default pen radius is 0.002, matching StdDraw. This value is interpreted as a fraction of the canvas width (i.e., relative to canvas size).
const DIAMETER_SCALE: f32 = 2.0;

// Colors
pub const BLACK: u32 = 0x000000;
pub const WHITE: u32 = 0xFFFFFF;
pub const RED: u32 = 0xFF0000;
pub const GREEN: u32 = 0x00FF00;
pub const BLUE: u32 = 0x0000FF;
pub const CYAN: u32 = 0x00FFFF;
pub const MAGENTA: u32 = 0xFF00FF;
pub const YELLOW: u32 = 0xFFFF00;

struct GlobalState {
    pixmap: Pixmap,
    width: usize,
    height: usize,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    pen_color: Color,
    pen_radius: f32,
    defer_update: bool,
    buffer: Vec<u32>, // Reusable buffer for rendering
}

impl GlobalState {
    fn new() -> Self {
        let width = DEFAULT_SIZE;
        let height = DEFAULT_SIZE;
        let mut pixmap =
            Pixmap::new(width as u32, height as u32).expect("Failed to create initial pixmap");
        pixmap.fill(Color::WHITE);

        GlobalState {
            pixmap,
            width,
            height,
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
            pen_color: Color::BLACK,
            pen_radius: DEFAULT_PEN_RADIUS,
            defer_update: false,
            buffer: vec![0; width * height],
        }
    }

    fn scale_x(&self, x: f64) -> f32 {
        let factor = self.width as f64 / (self.xmax - self.xmin);
        ((x - self.xmin) * factor) as f32
    }

    fn scale_y(&self, y: f64) -> f32 {
        let factor = self.height as f64 / (self.ymax - self.ymin);
        // Y is inverted in screen coordinates (0 is top)
        ((self.ymax - y) * factor) as f32
    }

    fn factor_x(&self) -> f32 {
        (self.width as f64 / (self.xmax - self.xmin)) as f32
    }

    fn factor_y(&self) -> f32 {
        (self.height as f64 / (self.ymax - self.ymin)) as f32
    }

    fn update_buffer(&mut self) {
        // Resize buffer if needed
        if self.buffer.len() != self.width * self.height {
            self.buffer.resize(self.width * self.height, 0);
        }

        // Convert pixmap to u32 buffer for minifb (ARGB -> 0RGB or similar)
        // tiny-skia uses Premultiplied RGBA8888. minifb expects 00RRGGBB.
        let pixels = self.pixmap.pixels();
        for (i, p) in pixels.iter().enumerate() {
            let r = p.red();
            let g = p.green();
            let b = p.blue();
            self.buffer[i] = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }
}

static STATE: LazyLock<Mutex<GlobalState>> = LazyLock::new(|| Mutex::new(GlobalState::new()));

thread_local! {
    static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

// Helper to convert u32 RGB to Color
fn u32_to_color(rgb: u32) -> Color {
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    Color::from_rgba8(r, g, b, 255)
}

// --- Public API ---

pub fn set_canvas_size(w: usize, h: usize) -> Result<(), String> {
    let mut state = STATE.lock().unwrap();
    state.width = w;
    state.height = h;
    state.pixmap = Pixmap::new(w as u32, h as u32)
        .ok_or_else(|| format!("Failed to create pixmap of size {}x{}", w, h))?;
    state.pixmap.fill(Color::WHITE);
    // Window will be recreated in show() if size mismatches
    Ok(())
}

pub fn set_x_scale(min: f64, max: f64) {
    let mut state = STATE.lock().unwrap();
    state.xmin = min;
    state.xmax = max;
}

pub fn set_y_scale(min: f64, max: f64) {
    let mut state = STATE.lock().unwrap();
    state.ymin = min;
    state.ymax = max;
}

pub fn set_pen_color(rgb: u32) {
    let mut state = STATE.lock().unwrap();
    state.pen_color = u32_to_color(rgb);
}

pub fn set_pen_radius(r: f32) {
    let mut state = STATE.lock().unwrap();
    state.pen_radius = r;
}

pub fn clear(rgb: u32) {
    let mut state = STATE.lock().unwrap();
    let color = u32_to_color(rgb);
    state.pixmap.fill(color);
    // No update_window() here, wait for show()
}

pub fn line(x0: f64, y0: f64, x1: f64, y1: f64) {
    let mut state = STATE.lock().unwrap();

    let sx0 = state.scale_x(x0);
    let sy0 = state.scale_y(y0);
    let sx1 = state.scale_x(x1);
    let sy1 = state.scale_y(y1);

    let mut pb = PathBuilder::new();
    pb.move_to(sx0, sy0);
    pb.line_to(sx1, sy1);
    let path = match pb.finish() {
        Some(p) => p,
        None => return, // Invalid path
    };

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    // Calculate stroke width based on pen radius and coordinate scale
    let stroke_width = state.pen_radius * state.factor_x() * DIAMETER_SCALE;

    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0);

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}

pub fn point(x: f64, y: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);

    // StdDraw's pen radius is a fraction of the canvas width.
    // A point is drawn as a filled circle of radius `pen_radius * canvas_width`.
    let radius = state.pen_radius * state.width as f32;
    let radius = radius.max(1.0);

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    let path = match PathBuilder::from_circle(sx, sy, radius) {
        Some(p) => p,
        None => return,
    };

    state.pixmap.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );
}

pub fn circle(x: f64, y: f64, r: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);
    // Circle radius r is in user coordinates
    let sr = r as f32 * state.factor_x();

    let path = match PathBuilder::from_circle(sx, sy, sr) {
        Some(p) => p,
        None => return,
    };

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    // Stroke width for circle outline is based on pen radius (fraction of canvas)
    let stroke_width = state.pen_radius * state.width as f32 * DIAMETER_SCALE;
    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0);

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}

pub fn filled_circle(x: f64, y: f64, r: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);
    let sr = r as f32 * state.factor_x();

    let path = match PathBuilder::from_circle(sx, sy, sr) {
        Some(p) => p,
        None => return,
    };

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    state.pixmap.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );
}

pub fn rectangle(x: f64, y: f64, half_width: f64, half_height: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x - half_width);
    let sy = state.scale_y(y + half_height); // Top-left Y (since Y is inverted)
    let w = (half_width * 2.0) as f32 * state.factor_x();
    let h = (half_height * 2.0) as f32 * state.factor_y();

    let rect = match tiny_skia::Rect::from_xywh(sx, sy, w, h) {
        Some(r) => r,
        None => return,
    };
    let path = PathBuilder::from_rect(rect);

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    let stroke_width = state.pen_radius * state.width as f32 * DIAMETER_SCALE;
    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0);

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}

pub fn filled_rectangle(x: f64, y: f64, half_width: f64, half_height: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x - half_width);
    let sy = state.scale_y(y + half_height);
    let w = (half_width * 2.0) as f32 * state.factor_x();
    let h = (half_height * 2.0) as f32 * state.factor_y();

    let rect = match tiny_skia::Rect::from_xywh(sx, sy, w, h) {
        Some(r) => r,
        None => return,
    };
    let path = PathBuilder::from_rect(rect);

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    state.pixmap.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );
}

pub fn text(_x: f64, _y: f64, _s: &str) {
    // Text rendering requires a font. tiny-skia doesn't do text layout itself easily without a font.
    // For now, we might skip text or use a very simple bitmap font if needed, or integrate rusttype/fontdue.
    // Leaving as TODO for this iteration.
    println!("Text rendering not yet implemented");
}

pub fn show(timeout_ms: u64) {
    let mut state = STATE.lock().unwrap();
    state.defer_update = false; // Enable updates

    // Update buffer from pixmap
    state.update_buffer();
    let width = state.width;
    let height = state.height;
    let buffer = state.buffer.clone(); // Clone buffer to release lock before window update?
                                       // No, we can hold lock, but window update might be slow.
                                       // But we need to release lock if we want other threads to draw?
                                       // Actually, minifb update is fast enough usually.
                                       // But `show` also sleeps. We MUST release lock before sleeping.

    drop(state); // Release lock before window ops and sleep

    WINDOW.with(|cell| {
        let mut win_opt = cell.borrow_mut();

        // Check if window needs creation or resizing
        let needs_recreate = if let Some(win) = win_opt.as_ref() {
            let (w, h) = win.get_size();
            w != width || h != height
        } else {
            true
        };

        if needs_recreate {
            let mut window = Window::new("StdDraw", width, height, WindowOptions::default())
                .unwrap_or_else(|e| {
                    panic!("Failed to create window: {}", e);
                });
            // Limit to max ~60 fps update rate
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            *win_opt = Some(window);
        }

        if let Some(win) = win_opt.as_mut() {
            win.update_with_buffer(&buffer, width, height).unwrap();
        }
    });

    if timeout_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms));
    }
}

pub fn is_open() -> bool {
    WINDOW.with(|cell| {
        if let Some(win) = cell.borrow().as_ref() {
            win.is_open()
        } else {
            false
        }
    })
}

pub fn is_key_down(key: Key) -> bool {
    WINDOW.with(|cell| {
        if let Some(win) = cell.borrow().as_ref() {
            win.is_key_down(key)
        } else {
            false
        }
    })
}

pub fn enable_double_buffering() {
    let mut state = STATE.lock().unwrap();
    state.defer_update = true;
}

pub fn disable_double_buffering() {
    let mut state = STATE.lock().unwrap();
    state.defer_update = false;
    // We should trigger an update, but we can't call show() here easily without timeout.
    // And we can't access window if we are not on main thread.
    // So we just set the flag. The next show() will update.
    // Or we could force an update if we are on main thread?
    // For now, just set flag.
}
