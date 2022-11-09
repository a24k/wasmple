use std::alloc::Layout;
use std::fmt::Debug;

use crate::console;

pub type BufferPtr = usize;

#[derive(Debug)]
pub struct Buffer {
    ptr: BufferPtr,
    layout: Layout,
}

impl Buffer {
    pub(super) fn alloc<T>(length: usize) -> Option<Self> {
        if length == 0 {
            return None;
        }

        let align = std::mem::align_of::<T>();
        let unit = std::mem::size_of::<T>();

        let layout = Layout::from_size_align(length * unit, align).ok()?;

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        console::debug(format!(
            "[wasm] Buffer::alloc ptr = {} layout = {:?}",
            ptr as usize, layout
        ));

        match ptr.is_null() {
            true => None,
            false => Some(Self {
                ptr: ptr as BufferPtr,
                layout,
            }),
        }
    }

    pub fn ptr(&self) -> BufferPtr {
        self.ptr
    }

    pub fn length<T>(&self) -> usize {
        let unit = std::mem::size_of::<T>();
        self.layout.size() / unit
    }

    pub fn slice<T>(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.length::<T>()) }
    }

    pub fn slice_mut<T>(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.length::<T>()) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr as *mut u8, self.layout) }
        console::debug(format!("[wasm] dealloc ptr = {}", self.ptr));
    }
}
