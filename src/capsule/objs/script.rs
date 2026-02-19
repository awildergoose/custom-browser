use std::sync::Arc;

use crate::capsule::obj::{CapsuleObject, CapsuleObjectBase};

#[derive(Debug, Default, Clone)]
pub struct CSScript {
    base: Arc<CapsuleObjectBase>,
    pub code: String,
}

impl CSScript {
    #[must_use]
    pub fn new(code: String) -> Self {
        Self {
            code,
            base: Arc::default(),
        }
    }
}

impl CapsuleObject for CSScript {
    fn base(&self) -> Arc<CapsuleObjectBase> {
        self.base.clone()
    }

    fn render(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
