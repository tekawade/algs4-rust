use lazy_static::lazy_static;
use minifb::{Key, Window, WindowOptions};
use std::sync::Mutex;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

// Default settings
const DEFAULT_SIZE: usize = 512;
const DEFAULT_PEN_RADIUS: f32 = 0.002; // Scaled relative to canvas size usually, but here absolute for now? No, StdDraw is relative.
                                       // StdDraw default pen radius is 0.002.

// Colors
pub const BLACK: u32 = 0x000000;
pub const WHITE: u32 = 0xFFFFFF;
pub const RED: u32 = 0xFF0000;
pub const GREEN: u32 = 0x00FF00;
pub const BLUE: u32 = 0x0000FF;
pub const CYAN: u32 = 0x00FFFF;
pub const MAGENTA: u32 = 0xFF00FF;
pub const YELLOW: u32 = 0xFFFF00;

struct ThreadSafeWindow(Window);

unsafe impl Send for ThreadSafeWindow {}
unsafe impl Sync for ThreadSafeWindow {}

struct GlobalState {
    window: Option<ThreadSafeWindow>,
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
}

impl GlobalState {
    fn new() -> Self {
        let width = DEFAULT_SIZE;
        let height = DEFAULT_SIZE;
        let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        pixmap.fill(Color::WHITE);

        GlobalState {
            window: None,
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
        }
    }

    fn init_window(&mut self) {
        if self.window.is_none() {
            let mut window =
                Window::new("StdDraw", self.width, self.height, WindowOptions::default())
                    .unwrap_or_else(|e| {
                        panic!("{}", e);
                    });

            // Limit to max ~60 fps update rate
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            self.window = Some(ThreadSafeWindow(window));
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

    fn update_window(&mut self) {
        if self.defer_update {
            return;
        }
        self.init_window();

        // Convert pixmap to u32 buffer for minifb (ARGB -> 0RGB or similar)
        // tiny-skia uses Premultiplied RGBA8888. minifb expects 00RRGGBB.
        // We need to convert.
        let buffer: Vec<u32> = self
            .pixmap
            .pixels()
            .iter()
            .map(|p| {
                let r = p.red();
                let g = p.green();
                let b = p.blue();
                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();

        if let Some(wrapper) = &mut self.window {
            wrapper
                .0
                .update_with_buffer(&buffer, self.width, self.height)
                .unwrap();
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<GlobalState> = Mutex::new(GlobalState::new());
}

// --- Public API ---

pub fn set_canvas_size(w: usize, h: usize) {
    let mut state = STATE.lock().unwrap();
    state.width = w;
    state.height = h;
    state.pixmap = Pixmap::new(w as u32, h as u32).unwrap();
    state.pixmap.fill(Color::WHITE);
    state.window = None; // Force recreation
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
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    state.pen_color = Color::from_rgba8(r, g, b, 255);
}

pub fn set_pen_radius(r: f32) {
    let mut state = STATE.lock().unwrap();
    state.pen_radius = r;
}

pub fn clear(rgb: u32) {
    let mut state = STATE.lock().unwrap();
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    let color = Color::from_rgba8(r, g, b, 255);
    state.pixmap.fill(color);
    state.update_window();
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
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    // Calculate stroke width based on pen radius and canvas size
    // StdDraw defines pen radius as fraction of canvas size (usually width)
    // But here we have independent scales. Let's approximate using width.
    let stroke_width = state.pen_radius * state.width as f32 * 2.0; // *2 because radius vs diameter? StdDraw says "radius".

    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0); // At least 1 pixel

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    state.update_window();
}

pub fn point(x: f64, y: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);

    let radius = state.pen_radius * state.width as f32;
    let radius = radius.max(1.0);

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    let path = PathBuilder::from_circle(sx, sy, radius).unwrap();
    state.pixmap.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );
    state.update_window();
}

pub fn circle(x: f64, y: f64, r: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);
    // Assuming uniform scaling for circle radius, or taking X scale
    let sr = r as f32 * state.factor_x();

    let path = PathBuilder::from_circle(sx, sy, sr).unwrap();

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    let stroke_width = state.pen_radius * state.width as f32 * 2.0;
    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0);

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    state.update_window();
}

pub fn filled_circle(x: f64, y: f64, r: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x);
    let sy = state.scale_y(y);
    let sr = r as f32 * state.factor_x();

    let path = PathBuilder::from_circle(sx, sy, sr).unwrap();

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
    state.update_window();
}

pub fn rectangle(x: f64, y: f64, half_width: f64, half_height: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x - half_width);
    let sy = state.scale_y(y + half_height); // Top-left Y (since Y is inverted)
    let w = (half_width * 2.0) as f32 * state.factor_x();
    let h = (half_height * 2.0) as f32 * state.factor_y();

    let rect = tiny_skia::Rect::from_xywh(sx, sy, w, h).unwrap();
    let path = PathBuilder::from_rect(rect);

    let mut paint = Paint::default();
    paint.set_color(state.pen_color);
    paint.anti_alias = true;

    let stroke_width = state.pen_radius * state.width as f32 * 2.0;
    let mut stroke = Stroke::default();
    stroke.width = stroke_width.max(1.0);

    state
        .pixmap
        .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    state.update_window();
}

pub fn filled_rectangle(x: f64, y: f64, half_width: f64, half_height: f64) {
    let mut state = STATE.lock().unwrap();
    let sx = state.scale_x(x - half_width);
    let sy = state.scale_y(y + half_height);
    let w = (half_width * 2.0) as f32 * state.factor_x();
    let h = (half_height * 2.0) as f32 * state.factor_y();

    let rect = tiny_skia::Rect::from_xywh(sx, sy, w, h).unwrap();
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
    state.update_window();
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
    state.update_window();

    // Handle window events (like close)
    if let Some(wrapper) = &mut state.window {
        if !wrapper.0.is_open() && !wrapper.0.is_key_down(Key::Escape) {
            // Window closed
        }
        // minifb update is called in update_window
    }

    if timeout_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms));
    }
}

pub fn enable_double_buffering() {
    let mut state = STATE.lock().unwrap();
    state.defer_update = true;
}

pub fn disable_double_buffering() {
    let mut state = STATE.lock().unwrap();
    state.defer_update = false;
    state.update_window();
}
