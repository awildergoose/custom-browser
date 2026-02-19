use std::sync::Arc;

use macroquad::color::WHITE;

use crate::{
    capsule::obj::{CapsuleObject, CapsuleObjectBase, CapsuleObjectChildren},
    layout::styling::Styling,
    renderer::text::draw_text_top_left,
};

#[derive(Debug, Default)]
pub struct CSText {
    base: Arc<CapsuleObjectBase>,
    pub text: String,
}

impl CSText {
    #[must_use]
    pub fn new(text: String, children: CapsuleObjectChildren, style: Arc<Styling>) -> Self {
        Self {
            text,
            base: CapsuleObjectBase::new(children, style),
        }
    }
}

impl CapsuleObject for CSText {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {
        let computed = self.base.computed_style.read();
        draw_text_top_left(
            &self.text,
            computed.x,
            computed.y,
            self.base.style.font_size.into(),
            self.base.style.color.unwrap_or(WHITE),
        );
    }
}
