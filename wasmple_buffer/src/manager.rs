use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use wasmple_console::debug;

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
        debug!("[wasm] dump {:?}", self);

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
        debug!("[wasm] dump {:?}", self);

        removed
    }

    pub fn clear(&mut self) {
        self.ring.clear();
        debug!("[wasm] dump {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{Arc, Buffer, BufferManager, Mutex};

    #[rstest]
    fn manager_lock() {
        let manager = BufferManager::lock();
        std::mem::drop(manager);
    }

    #[rstest]
    #[case( 1, BufferManager::lock().alloc::<i8> (1))]
    #[case( 2, BufferManager::lock().alloc::<i8> (2))]
    #[case( 3, BufferManager::lock().alloc::<i8> (3))]
    #[case( 4, BufferManager::lock().alloc::<i8> (4))]
    #[case( 8, BufferManager::lock().alloc::<i8> (8))]
    #[case( 8, BufferManager::lock().alloc::<u8> (8))]
    #[case(16, BufferManager::lock().alloc::<i16>(8))]
    #[case(16, BufferManager::lock().alloc::<u16>(8))]
    #[case(32, BufferManager::lock().alloc::<i32>(8))]
    #[case(32, BufferManager::lock().alloc::<u32>(8))]
    #[case(64, BufferManager::lock().alloc::<i64>(8))]
    #[case(64, BufferManager::lock().alloc::<u64>(8))]
    #[case(32, BufferManager::lock().alloc::<f32>(8))]
    #[case(64, BufferManager::lock().alloc::<f64>(8))]
    fn manager_alloc_length_dealloc(
        #[case] expected: usize,
        #[case] input: Option<Arc<Mutex<Buffer>>>,
    ) {
        match input {
            None => panic!("input: Option<Arc<Mutex<Buffer>>> will be Some."),
            Some(arc) => {
                let buf = arc.lock().unwrap();
                let ptr = buf.ptr();
                assert_eq!(expected, buf.length::<u8>());
                BufferManager::lock().dealloc(ptr);
            }
        }
    }

    #[rstest]
    #[case(BufferManager::lock().alloc::<i8> (0))]
    #[case(BufferManager::lock().alloc::<u8> (0))]
    #[case(BufferManager::lock().alloc::<i16>(0))]
    #[case(BufferManager::lock().alloc::<u16>(0))]
    #[case(BufferManager::lock().alloc::<i32>(0))]
    #[case(BufferManager::lock().alloc::<u32>(0))]
    #[case(BufferManager::lock().alloc::<i64>(0))]
    #[case(BufferManager::lock().alloc::<u64>(0))]
    #[case(BufferManager::lock().alloc::<f32>(0))]
    #[case(BufferManager::lock().alloc::<f64>(0))]
    fn manager_alloc_with_zero_length(#[case] input: Option<Arc<Mutex<Buffer>>>) {
        if let Some(_) = input {
            panic!("input: Option<Arc<Mutex<Buffer>>> must be None.");
        }
    }

    #[rstest]
    #[case(BufferManager::lock().alloc::<i8> (8))]
    #[case(BufferManager::lock().alloc::<u8> (8))]
    #[case(BufferManager::lock().alloc::<i16>(8))]
    #[case(BufferManager::lock().alloc::<u16>(8))]
    #[case(BufferManager::lock().alloc::<i32>(8))]
    #[case(BufferManager::lock().alloc::<u32>(8))]
    #[case(BufferManager::lock().alloc::<i64>(8))]
    #[case(BufferManager::lock().alloc::<u64>(8))]
    #[case(BufferManager::lock().alloc::<f32>(8))]
    #[case(BufferManager::lock().alloc::<f64>(8))]
    fn manager_alloc_get_dealloc_get(#[case] input: Option<Arc<Mutex<Buffer>>>) {
        match input {
            None => panic!("input: Option<Arc<Mutex<Buffer>>> will be Some."),
            Some(arc) => {
                let ptr = arc.lock().unwrap().ptr();

                if let None = BufferManager::lock().get(ptr) {
                    panic!("input: Option<Arc<Mutex<Buffer>>> will be Some.");
                }

                BufferManager::lock().dealloc(ptr);

                if let Some(_) = BufferManager::lock().get(ptr) {
                    panic!("input: Option<Arc<Mutex<Buffer>>> will be Null.");
                }
            }
        }
    }
}
