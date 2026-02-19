use serde::{Deserialize, Serialize};
use stretch::style::JustifyContent;
use strum::{AsRefStr, EnumString};

#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Default, EnumString, AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum COJustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl COJustifyContent {
    #[must_use]
    pub const fn as_stretch(&self) -> JustifyContent {
        match self {
            Self::FlexStart => JustifyContent::FlexStart,
            Self::FlexEnd => JustifyContent::FlexEnd,
            Self::Center => JustifyContent::Center,
            Self::SpaceBetween => JustifyContent::SpaceBetween,
            Self::SpaceAround => JustifyContent::SpaceAround,
            Self::SpaceEvenly => JustifyContent::SpaceEvenly,
        }
    }
}
