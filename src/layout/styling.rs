use stretch::style::{AlignItems, Dimension, JustifyContent};

#[derive(Debug, Default, Clone)]
pub struct Styling {
    pub align: AlignItems,
    pub justify: JustifyContent,
    pub width: Dimension,
    pub height: Dimension,
}
