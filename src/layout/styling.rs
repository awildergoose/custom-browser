use macroquad::color::{Color, WHITE};
use stretch::style::{AlignItems, Dimension, JustifyContent};

use crate::renderer::constants::DEFAULT_TEXT_SIZE;

#[derive(Debug, Clone)]
pub struct Styling {
    pub align: AlignItems,
    pub justify: JustifyContent,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub font_size: u16,
    pub color: Color,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            align: AlignItems::default(),
            justify: JustifyContent::default(),
            width: None,
            height: None,
            font_size: DEFAULT_TEXT_SIZE,
            color: WHITE,
        }
    }
}
