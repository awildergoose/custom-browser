#[derive(Debug)]
pub struct CapsuleObjectEvent {
    pub name: String,
    pub callback: Option<String>,
}

impl CapsuleObjectEvent {
    #[must_use]
    pub const fn new(name: String, callback: Option<String>) -> Self {
        Self { name, callback }
    }
}
