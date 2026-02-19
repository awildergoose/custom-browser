use std::sync::Arc;

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

    fn render(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
