use super::{BufferConverter, BufferManager, BufferPtr};

impl BufferConverter<String> for String {
    fn from(ptr: BufferPtr) -> Option<String> {
        let arc = BufferManager::lock().get(ptr)?;

        let buf = arc.lock().unwrap();
        let slice = buf.slice::<u16>();

        String::from_utf16(slice).ok()
    }

    fn into(&self) -> Option<BufferPtr> {
        let utf16: Vec<u16> = self.encode_utf16().collect();

        let arc = BufferManager::lock().alloc::<u16>(utf16.len())?;

        let mut buf = arc.lock().unwrap();
        let slice = buf.slice_mut::<u16>();

        slice.copy_from_slice(&utf16[..]);

        Some(buf.ptr())
    }
}
