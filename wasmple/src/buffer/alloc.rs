use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::console;

use super::buffer::{Buffer, BufferPtr};

#[derive(Debug)]
pub enum BufferTypes {
    U16(Buffer<u16>),
}

impl BufferTypes {
    pub fn slice(&self) -> SliceTypes {
        match self {
            BufferTypes::U16(buf) => SliceTypes::U16(buf.slice()),
        }
    }

    pub fn slice_mut(&mut self) -> SliceTypes {
        match self {
            BufferTypes::U16(buf) => SliceTypes::U16M(buf.slice_mut()),
        }
    }
}

#[derive(Debug)]
pub enum SliceTypes<'a> {
    U16(&'a [u16]),
    U16M(&'a mut [u16]),
}

pub static BUFFERS: Lazy<Mutex<HashMap<BufferPtr, BufferTypes>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn alloc_u16(length: usize) -> BufferPtr {
    let mut buffers = BUFFERS.lock().unwrap();
    let buf: Buffer<u16> = Buffer::new(length).unwrap();
    let ptr = buf.ptr();
    buffers.insert(ptr, BufferTypes::U16(buf));
    console::debug(format!("[wasm] buffer::BUFFERS\t{:?}", buffers));
    ptr
}

pub fn length(ptr: BufferPtr) -> usize {
    let buffers = BUFFERS.lock().unwrap();
    match buffers.get(&ptr).unwrap() {
        BufferTypes::U16(buf) => buf.length(),
    }
}

pub fn dealloc(ptr: BufferPtr) {
    let mut buffers = BUFFERS.lock().unwrap();
    buffers.remove(&ptr);
    console::debug(format!("[wasm] buffer::BUFFERS\t{:?}", buffers));
}

/*
pub fn slice(ptr: BufferPtr) -> SliceTypes<'static> {
    let buffers = BUFFERS.lock().unwrap();
    buffers.get(&ptr).unwrap().slice()
}
*/
