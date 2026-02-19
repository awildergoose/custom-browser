use std::sync::Arc;

use macroquad::shapes::draw_rectangle;

use crate::capsule::obj::{CapsuleObject, CapsuleObjectBase, CapsuleObjectCreationContext};

// like a div
#[derive(Debug, Default)]
pub struct CSObj {
    base: Arc<CapsuleObjectBase>,
}

impl CSObj {
    #[must_use]
    pub fn new(ctx: CapsuleObjectCreationContext) -> Self {
        Self {
            base: CapsuleObjectBase::new(ctx),
        }
    }
}

impl CapsuleObject for CSObj {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {
        let style = self.base.computed_style.read();

        if let Some(color) = self.base.style.color {
            draw_rectangle(style.x, style.y, style.width, style.height, color);
        }
    }
}
