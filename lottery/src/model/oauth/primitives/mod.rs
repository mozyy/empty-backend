use tokio::sync::MutexGuard;

pub mod authorizer;
pub mod issuer;
pub mod registrar;

pub struct Guard<'a, T>(MutexGuard<'a, T>);

impl<'a, T> From<MutexGuard<'a, T>> for Guard<'a, T> {
    fn from(value: MutexGuard<'a, T>) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
