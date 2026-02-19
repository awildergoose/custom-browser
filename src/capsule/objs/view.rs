use std::sync::Arc;

use crate::capsule::obj::{CapsuleObject, CapsuleObjectBase};

#[derive(Debug, Default)]
pub struct CapsuleView {
    base: Arc<CapsuleObjectBase>,
}

impl CapsuleObject for CapsuleView {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {}
}
