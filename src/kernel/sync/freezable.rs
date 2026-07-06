use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;

pub struct FreezableToken {
    // prevent Clone, Copy, Send, and Sync
    _marker: PhantomData<*mut ()>,
}

impl FreezableToken {
    pub const unsafe fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn write<T, U>(&mut self, value: &Freezable<T>, write_fn: impl Fn(&mut T) -> U) -> U {
        let value = unsafe { &mut *value.value.get() };
        write_fn(value)
    }

    pub fn forget(self) {
        mem::forget(self);
    }
}

impl Drop for FreezableToken {
    fn drop(&mut self) {
        panic!("token must be explicitly forget");
    }
}

pub struct Freezable<T> {
    value: UnsafeCell<T>,
}

/// SAFETY: after a `Freezable<T>` is shared, mutation through its `UnsafeCell`
/// is allowed only while holding the unique `FreezableToken`; callers of
/// `FreezableToken::new` must ensure that no such token exists during the
/// frozen shared phase. Shared access exposes `&T`, so `T` must be `Sync`.
unsafe impl<T: Sync> Sync for Freezable<T> {}

impl<T> Deref for Freezable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

impl<T> Freezable<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
}
