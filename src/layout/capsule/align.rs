use serde::{Deserialize, Serialize};
use stretch::style::AlignItems;
use strum::{AsRefStr, EnumString};

#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Default, EnumString, AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum COAlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    #[default]
    Stretch,
}

impl COAlignItems {
    #[must_use]
    pub const fn as_stretch(&self) -> AlignItems {
        match self {
            Self::FlexStart => AlignItems::FlexStart,
            Self::FlexEnd => AlignItems::FlexEnd,
            Self::Center => AlignItems::Center,
            Self::Baseline => AlignItems::Baseline,
            Self::Stretch => AlignItems::Stretch,
        }
    }
}
