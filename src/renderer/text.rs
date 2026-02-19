use macroquad::{
    color::Color,
    text::{draw_text, measure_text},
};

pub fn draw_text_top_left(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x, y + dims.offset_y, font_size, color);
}
