use std::{fmt::Debug, sync::Arc};

use orx_concurrent_vec::ConcurrentVec;
use parking_lot::RwLock;

use crate::layout::{computed::ComputedStyling, styling::Styling};

pub type BoxedCapsuleObject = Box<dyn CapsuleObject + Sync + Send>;
pub type CapsuleObjectChildren = Arc<ConcurrentVec<BoxedCapsuleObject>>;

pub trait CapsuleObject: Debug {
    fn base(&self) -> Arc<CapsuleObjectBase>;
    fn render(&self);
}

#[derive(Debug, Default)]
pub struct CapsuleObjectBase {
    pub children: CapsuleObjectChildren,
    pub style: Arc<Styling>,
    pub computed_style: Arc<RwLock<ComputedStyling>>,
}

impl CapsuleObjectBase {
    pub fn new(children: CapsuleObjectChildren, style: Arc<Styling>) -> Arc<Self> {
        Arc::new(Self {
            children,
            style,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default)]
pub struct CapsuleMeta {
    pub title: String,
}

#[derive(Debug, Default)]
pub struct Capsule {
    pub meta: CapsuleMeta,
    pub view: CapsuleView,
}

// view
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
