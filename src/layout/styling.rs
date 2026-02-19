use macroquad::color::Color;
use mlua::UserData;
use serde::Serialize;
use stretch::style::{AlignItems, Dimension, FlexDirection, JustifyContent};

use crate::renderer::constants::DEFAULT_TEXT_SIZE;

#[derive(Debug, Clone)]
pub struct Styling {
    pub align: AlignItems,
    pub justify: JustifyContent,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub font_size: u16,
    pub color: Option<Color>,
    pub flex_direction: Option<FlexDirection>,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            align: AlignItems::default(),
            justify: JustifyContent::default(),
            width: None,
            height: None,
            font_size: DEFAULT_TEXT_SIZE,
            color: None,
            flex_direction: None,
        }
    }
}

impl UserData for Styling {}

impl Serialize for Styling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit_variant("align", 0, "variant")
    }
}
