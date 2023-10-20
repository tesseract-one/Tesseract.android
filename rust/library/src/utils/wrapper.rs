pub (crate) struct Wrapper<T> {
    pub inner: T
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Self { inner: value }
    }
}