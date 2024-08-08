use std::{
    ops::{Deref, DerefMut},
    thread::panicking,
};

pub trait Buffer {
    fn size(&self) -> u8;
    fn read(&self, dst: &mut [u8]);
}

pub trait RwBuffer: Buffer {
    fn write(&mut self, src: &[u8]);
}

pub trait LockableBuffer<F: FnOnce()>: RwBuffer {
    fn lock(&mut self) -> BufferGuard<'_, F>;
}

#[derive(Debug)]
pub struct BufferGuard<'a, F: FnOnce()> {
    inner: &'a mut [u8],
    on_drop: Option<F>,
}

impl<'a, F: FnOnce()> BufferGuard<'a, F> {
    pub fn new(inner: &'a mut [u8], on_drop: F) -> Self {
        let on_drop = Some(on_drop);
        Self { inner, on_drop }
    }
}

impl<'a, F: FnOnce()> Deref for BufferGuard<'a, F> {
    type Target = &'a mut [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, F: FnOnce()> DerefMut for BufferGuard<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<F: FnOnce()> Drop for BufferGuard<'_, F> {
    fn drop(&mut self) {
        if !panicking() {
            (self.on_drop.take().unwrap())();
        }
    }
}
