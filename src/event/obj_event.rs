#[derive(Debug)]
pub struct CapsuleObjectEvent {
    pub name: String,
    pub callback: String,
}

impl CapsuleObjectEvent {
    #[must_use]
    pub fn new<S>(name: S, callback: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            callback: callback.into(),
        }
    }
}
