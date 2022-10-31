use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::console;

use super::{Buffer, BufferPtr};

#[derive(Debug)]
pub struct BufferManager {
    ring: HashMap<BufferPtr, Arc<Mutex<Buffer>>>,
}

impl BufferManager {
    pub fn lock() -> MutexGuard<'static, BufferManager> {
        static MANAGER: OnceCell<Mutex<BufferManager>> = OnceCell::new();
        MANAGER
            .get_or_init(|| {
                Mutex::new(BufferManager {
                    ring: HashMap::new(),
                })
            })
            .lock()
            .unwrap()
    }

    pub fn alloc<T>(&mut self, length: usize) -> BufferPtr {
        let buf = Buffer::alloc::<T>(length);
        match buf {
            None => 0,
            Some(buf) => {
                let ptr = buf.ptr();
                self.ring.insert(ptr, Arc::new(Mutex::new(buf)));
                console::debug(format!("[wasm] buffer::MANAGER {:?}", self));
                ptr
            }
        }
    }

    pub fn buffer(&self, ptr: BufferPtr) -> Arc<Mutex<Buffer>> {
        self.ring.get(&ptr).unwrap().clone()
    }

    pub fn length<T>(&self, ptr: BufferPtr) -> usize {
        self.buffer(ptr).lock().unwrap().length::<T>()
    }

    pub fn dealloc(&mut self, ptr: BufferPtr) {
        self.ring.remove(&ptr);
        console::debug(format!("[wasm] buffer::MANAGER {:?}", self));
    }
}
