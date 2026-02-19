use serde::{Deserialize, Serialize};
use stretch::style::FlexDirection;
use strum::{AsRefStr, EnumString};

#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Default, EnumString, AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum COFlexDirection {
    #[default]
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl COFlexDirection {
    #[must_use]
    pub const fn as_stretch(&self) -> FlexDirection {
        match self {
            Self::Row => FlexDirection::Row,
            Self::Column => FlexDirection::Column,
            Self::RowReverse => FlexDirection::RowReverse,
            Self::ColumnReverse => FlexDirection::ColumnReverse,
        }
    }
}
