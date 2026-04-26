use macroquad::{color::Color, shapes::draw_rectangle};

const PIXEL_SIZE: f32 = 2.0;

pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    draw_rounded_rect_asymmetric(x, y, w, h, r, r, color);
}

pub fn draw_rounded_rect_asymmetric(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    top_r: f32,
    bottom_r: f32,
    color: Color,
) {
    draw_scanlines(x, y, w, h, top_r, bottom_r, color, PIXEL_SIZE);
}

pub fn draw_circle_pixelated(cx: f32, cy: f32, r: f32, color: Color) {
    draw_rounded_rect(cx - r, cy - r, r * 2.0, r * 2.0, r, color);
}

fn draw_scanlines(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    top_r: f32,
    bottom_r: f32,
    color: Color,
    step: f32,
) {
    let top_r = top_r.min(w / 2.0).min(h / 2.0);
    let bottom_r = bottom_r.min(w / 2.0).min(h / 2.0);

    let y_steps = (h / step).ceil() as i32;
    for yi in 0..y_steps {
        let py = y + yi as f32 * step;
        let strip_mid = py + step * 0.5;

        let x_inset = if strip_mid < y + top_r && top_r > 0.0 {
            let dy = (y + top_r) - strip_mid;
            let inset = top_r - (top_r * top_r - dy.min(top_r).powi(2)).max(0.0).sqrt();
            ((inset - step * 0.5) / step).ceil() * step
        } else if strip_mid > y + h - bottom_r && bottom_r > 0.0 {
            let dy = strip_mid - (y + h - bottom_r);
            let inset = bottom_r
                - (bottom_r * bottom_r - dy.min(bottom_r).powi(2))
                    .max(0.0)
                    .sqrt();
            ((inset - step * 0.5) / step).ceil() * step
        } else {
            0.0
        };

        let strip_h = step.min(y + h - py);
        let strip_w = w - x_inset * 2.0;
        if strip_w > 0.0 {
            draw_rectangle(x + x_inset, py, strip_w, strip_h, color);
        }
    }
}
