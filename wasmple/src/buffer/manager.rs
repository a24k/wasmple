use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::console_debug;

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

    pub fn alloc<T>(&mut self, length: usize) -> Option<Arc<Mutex<Buffer>>> {
        let buf = Buffer::alloc::<T>(length)?;

        let ptr = buf.ptr();
        let arc = Arc::new(Mutex::new(buf));

        self.ring.insert(ptr, arc.clone());
        console_debug!("[wasm] dump {:?}", self);

        Some(arc)
    }

    pub fn get(&self, ptr: BufferPtr) -> Option<Arc<Mutex<Buffer>>> {
        Some(self.ring.get(&ptr)?.clone())
    }

    pub fn length<T>(&self, ptr: BufferPtr) -> usize {
        self.get(ptr)
            .map_or(0, |arc| arc.lock().unwrap().length::<T>())
    }

    pub fn dealloc(&mut self, ptr: BufferPtr) -> Option<Arc<Mutex<Buffer>>> {
        let removed = self.ring.remove(&ptr);
        console_debug!("[wasm] dump {:?}", self);

        removed
    }

    pub fn clear(&mut self) {
        self.ring.clear();
        console_debug!("[wasm] dump {:?}", self);
    }
}
