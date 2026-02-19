use std::sync::Arc;

use crate::{
    capsule::obj::{CapsuleObject, CapsuleObjectBase, CapsuleObjectCreationContext},
    impl_obj_traits,
};

#[derive(Debug, Default, Clone)]
pub struct CSView {
    base: Arc<CapsuleObjectBase>,
}

impl CSView {
    #[must_use]
    pub fn new(ctx: CapsuleObjectCreationContext) -> Self {
        Self {
            base: CapsuleObjectBase::new(ctx),
        }
    }
}

impl CapsuleObject for CSView {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl_obj_traits!(CSView);
