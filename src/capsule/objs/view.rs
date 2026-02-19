use std::sync::Arc;

use crate::{
    capsule::obj::{CapsuleObject, CapsuleObjectBase},
    impl_obj_traits,
};

#[derive(Debug, Default, Clone)]
pub struct CapsuleView {
    base: Arc<CapsuleObjectBase>,
}

impl CapsuleObject for CapsuleView {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl_obj_traits!(CapsuleView);
