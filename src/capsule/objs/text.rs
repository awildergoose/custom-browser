use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    capsule::obj::{ArcLock, CapsuleObject, CapsuleObjectBase, CapsuleObjectCreationContext},
    layout::capsule::color::WHITE,
    renderer::text::draw_text_top_left,
};

#[derive(Debug, Default)]
pub struct CSText {
    base: Arc<CapsuleObjectBase>,
    pub text: ArcLock<String>,
}

impl CSText {
    #[must_use]
    pub fn new(text: String, ctx: CapsuleObjectCreationContext) -> Self {
        Self {
            text: RwLock::new(text).into(),
            base: CapsuleObjectBase::new(ctx),
        }
    }

    pub fn set_text(&self, text: String) {
        *self.text.write() = text;
        // This doesn't work?
        self.base.style.write().set_dirty();
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
            &self.text.read(),
            computed.x,
            computed.y,
            style.font_size.into(),
            style.color.unwrap_or(WHITE).as_macroquad(),
        );
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
