use serde::{Deserialize, Serialize};
use stretch::style::Dimension;
use strum::{AsRefStr, EnumString};

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize, EnumString, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CODimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

impl CODimension {
    #[must_use]
    pub const fn as_stretch(&self) -> Dimension {
        match self {
            Self::Auto => Dimension::Auto,
            Self::Undefined => Dimension::Undefined,
            Self::Percent(v) => Dimension::Percent(*v),
            Self::Points(v) => Dimension::Points(*v),
        }
    }

    #[must_use]
    pub fn as_text(&self) -> String {
        match self {
            Self::Undefined => "undefined".to_owned(),
            Self::Auto => "auto".to_owned(),
            Self::Points(v) => format!("{v}"),
            Self::Percent(v) => format!("{}%", v * 100.0),
        }
    }
}
