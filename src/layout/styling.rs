use mlua::UserData;
use serde::{Deserialize, Serialize};

use crate::{
    layout::capsule::{
        align::COAlignItems, color::COColor, dimension::CODimension, flexdir::COFlexDirection,
        justify::COJustifyContent,
    },
    renderer::constants::DEFAULT_TEXT_SIZE,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Styling {
    pub align: COAlignItems,
    pub justify: COJustifyContent,
    pub width: Option<CODimension>,
    pub height: Option<CODimension>,
    pub font_size: u16,
    pub color: Option<COColor>,
    pub flex_direction: Option<COFlexDirection>,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            align: COAlignItems::default(),
            justify: COJustifyContent::default(),
            width: None,
            height: None,
            font_size: DEFAULT_TEXT_SIZE,
            color: None,
            flex_direction: None,
        }
    }
}

impl UserData for Styling {}
