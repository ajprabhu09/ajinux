use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn from(data: T) -> Self {
        Self {
            lock: AtomicBool::from(false),
            data: UnsafeCell::from(data),
        }
    }
    fn try_lock(&self) -> Result<MutexGuard<T>, &'static str> {
        if !self.lock.swap(true, Ordering::Acquire) {
            Ok(MutexGuard { mutex: self })
        } else {
            Err("could not acquire mutex")
        }
    }

    pub fn lock<'a>(&mut self) -> MutexGuard<T> {
        loop {
            if let Ok(mutex) = self.try_lock() {
                return mutex;
            }
        }
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe {
            self.data.get().drop_in_place();
        }
    }
}

unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let _a = self.mutex.lock.swap(false, Ordering::Release);
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
