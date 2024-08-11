use std::{
    ops::{Deref, DerefMut},
    thread::panicking,
};

pub trait Buffer<T> {
    fn size(&self) -> T;
    fn read(&self, dst: &mut [T]);
}

pub trait RwBuffer<T>: Buffer<T> {
    fn write(&mut self, src: &[u8]);
}

pub trait LockBuffer<T, F: FnOnce()>: RwBuffer<T> {
    fn lock(&mut self) -> BufferGuard<'_, T, F>;
}

#[derive(Debug)]
pub struct BufferGuard<'a, T, F: FnOnce()> {
    inner: &'a mut [T],
    on_drop: Option<F>,
}

impl<'a, T, F: FnOnce()> BufferGuard<'a, T, F> {
    pub fn new(inner: &'a mut [T], on_drop: F) -> Self {
        let on_drop = Some(on_drop);
        Self { inner, on_drop }
    }
}

impl<'a, T, F: FnOnce()> Deref for BufferGuard<'a, T, F> {
    type Target = &'a mut [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T, F: FnOnce()> DerefMut for BufferGuard<'a, T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, F: FnOnce()> Drop for BufferGuard<'_, T, F> {
    fn drop(&mut self) {
        if !panicking() {
            (self.on_drop.take().unwrap())();
        }
    }
}
