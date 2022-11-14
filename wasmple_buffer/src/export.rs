use super::BufferManager;
use super::BufferPtr;

enum T {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl From<u8> for T {
    fn from(t: u8) -> Self {
        match t {
            0 => T::I8,
            1 => T::U8,
            2 => T::I16,
            3 => T::U16,
            4 => T::I32,
            5 => T::U32,
            6 => T::I64,
            7 => T::U64,
            8 => T::F32,
            9 => T::F64,
            _ => T::U8,
        }
    }
}

#[no_mangle]
pub extern "C" fn buffer_alloc(t: u8, len: usize) -> BufferPtr {
    let arc = match T::from(t) {
        T::I8 => BufferManager::lock().alloc::<i8>(len),
        T::U8 => BufferManager::lock().alloc::<u8>(len),
        T::I16 => BufferManager::lock().alloc::<i16>(len),
        T::U16 => BufferManager::lock().alloc::<u16>(len),
        T::I32 => BufferManager::lock().alloc::<i32>(len),
        T::U32 => BufferManager::lock().alloc::<u32>(len),
        T::I64 => BufferManager::lock().alloc::<i64>(len),
        T::U64 => BufferManager::lock().alloc::<u64>(len),
        T::F32 => BufferManager::lock().alloc::<f32>(len),
        T::F64 => BufferManager::lock().alloc::<f64>(len),
    };
    arc.map_or(0, |arc| arc.lock().unwrap().ptr())
}

#[no_mangle]
pub extern "C" fn buffer_length(t: u8, ptr: BufferPtr) -> usize {
    match T::from(t) {
        T::I8 => BufferManager::lock().length::<i8>(ptr),
        T::U8 => BufferManager::lock().length::<u8>(ptr),
        T::I16 => BufferManager::lock().length::<i16>(ptr),
        T::U16 => BufferManager::lock().length::<u16>(ptr),
        T::I32 => BufferManager::lock().length::<i32>(ptr),
        T::U32 => BufferManager::lock().length::<u32>(ptr),
        T::I64 => BufferManager::lock().length::<i64>(ptr),
        T::U64 => BufferManager::lock().length::<u64>(ptr),
        T::F32 => BufferManager::lock().length::<f32>(ptr),
        T::F64 => BufferManager::lock().length::<f64>(ptr),
    }
}

#[no_mangle]
pub extern "C" fn buffer_dealloc(ptr: BufferPtr) {
    BufferManager::lock().dealloc(ptr);
}

#[no_mangle]
pub extern "C" fn buffer_clear() {
    BufferManager::lock().clear();
}
