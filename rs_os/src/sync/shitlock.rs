use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

pub struct Racy<T> {
    data: UnsafeCell<T>,
}

pub struct RacyGuard<'a, T> {
    mutex: &'a Racy<T>,
}

impl<T> Racy<T> {
    pub fn from(data: T) -> Self {
        Self {
            data: UnsafeCell::from(data),
        }
    }
    pub fn take(&self) -> RacyGuard<T> {
        RacyGuard { mutex: self }
    }
}

impl<T> Drop for Racy<T> {
    fn drop(&mut self) {
        unsafe {
            self.data.get().drop_in_place();
        }
    }
}

unsafe impl<T> Send for Racy<T> {}
unsafe impl<T> Sync for Racy<T> {}

impl<T> Drop for RacyGuard<'_, T> {
    fn drop(&mut self) {}
}

impl<T> Deref for RacyGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for RacyGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
