use std::alloc::Layout;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::console;

pub type BufferPtr = usize;

#[derive(Debug)]
pub struct Buffer<T> {
    length: usize,
    layout: Layout,
    ptr: BufferPtr,
    phantom: PhantomData<T>,
}

impl Buffer<u16> {
    pub fn new(length: usize) -> Option<Self> {
        Self::alloc(length)
    }
}

impl<T> Buffer<T> {
    fn alloc(length: usize) -> Option<Self> {
        let align = std::mem::align_of::<T>();
        let unit = std::mem::size_of::<T>();

        let layout = Layout::from_size_align(length * unit, align).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        console::debug(format!("[wasm] alloc {:?}", ptr));

        match ptr.is_null() {
            true => None,
            false => Some(Self {
                length,
                layout,
                ptr: ptr as BufferPtr,
                phantom: PhantomData,
            }),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn ptr(&self) -> BufferPtr {
        self.ptr as BufferPtr
    }

    pub fn slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.length) }
    }

    pub fn slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.length) }
    }

    fn dealloc(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr as *mut u8, self.layout) }
        console::debug(format!("[wasm] dealloc {:?}", self.ptr as *mut u8));
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.dealloc();
    }
}
