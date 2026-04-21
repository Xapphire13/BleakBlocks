use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_rectangle},
};

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
    let top_r = top_r.min(w / 2.0).min(h / 2.0);
    let bottom_r = bottom_r.min(w / 2.0).min(h / 2.0);
    draw_rectangle(x, y + top_r, w, h - top_r - bottom_r, color);
    draw_rectangle(x + top_r, y, w - top_r * 2.0, top_r, color);
    draw_rectangle(
        x + bottom_r,
        y + h - bottom_r,
        w - bottom_r * 2.0,
        bottom_r,
        color,
    );
    draw_circle(x + top_r, y + top_r, top_r, color);
    draw_circle(x + w - top_r, y + top_r, top_r, color);
    draw_circle(x + bottom_r, y + h - bottom_r, bottom_r, color);
    draw_circle(x + w - bottom_r, y + h - bottom_r, bottom_r, color);
}
