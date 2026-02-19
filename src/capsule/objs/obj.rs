use std::sync::Arc;

use macroquad::shapes::draw_rectangle;

use crate::{
    capsule::obj::{CapsuleObject, CapsuleObjectBase, CapsuleObjectChildren},
    layout::styling::Styling,
};

// like a div
#[derive(Debug, Default)]
pub struct CSObj {
    base: Arc<CapsuleObjectBase>,
}

impl CSObj {
    #[must_use]
    pub fn new(children: CapsuleObjectChildren, style: Arc<Styling>) -> Self {
        Self {
            base: CapsuleObjectBase::new(children, style),
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
