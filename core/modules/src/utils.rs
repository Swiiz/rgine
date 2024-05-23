use std::ops::{Deref, DerefMut};

/// Behave just like T but can be moved from a mutable reference
/// ONLY ONCE or it will panic (can be checked using `Take<T>::taken(&self)`)
pub struct Take<T>(Option<T>);

impl<T> Take<T> {
    pub fn new(v: T) -> Self {
        Self(Some(v))
    }

    pub fn taken(&self) -> bool {
        self.0.is_none()
    }

    pub fn take(&mut self) -> T {
        self.0.take().expect("Can't take a value twice")
    }
}

impl<T> Deref for Take<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<T> DerefMut for Take<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
