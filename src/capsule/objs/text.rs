use std::sync::Arc;

use macroquad::color::WHITE;

use crate::{
    capsule::obj::{CapsuleObject, CapsuleObjectBase, CapsuleObjectCreationContext},
    renderer::text::draw_text_top_left,
};

#[derive(Debug, Default)]
pub struct CSText {
    base: Arc<CapsuleObjectBase>,
    pub text: String,
}

impl CSText {
    #[must_use]
    pub fn new(text: String, ctx: CapsuleObjectCreationContext) -> Self {
        Self {
            text,
            base: CapsuleObjectBase::new(ctx),
        }
    }
}

impl CapsuleObject for CSText {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {
        let computed = self.base.computed_style.read();
        let style = self.base.style.read();
        draw_text_top_left(
            &self.text,
            computed.x,
            computed.y,
            style.font_size.into(),
            style.color.unwrap_or(WHITE),
        );
    }
}
