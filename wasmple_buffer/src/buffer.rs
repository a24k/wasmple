use std::alloc::Layout;
use std::fmt::Debug;

use wasmple_bridge::wasmple_bridge;
use wasmple_console::debug;

#[wasmple_bridge]
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
        debug!(
            "[wasm] Buffer::alloc ptr = {} layout = {:?}",
            ptr as usize, layout
        );

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
        debug!("[wasm] dealloc ptr = {}", self.ptr);
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::Buffer;

    #[rstest]
    #[case(Buffer::alloc::<i8> (8))]
    #[case(Buffer::alloc::<u8> (8))]
    #[case(Buffer::alloc::<i16>(8))]
    #[case(Buffer::alloc::<u16>(8))]
    #[case(Buffer::alloc::<i32>(8))]
    #[case(Buffer::alloc::<u32>(8))]
    #[case(Buffer::alloc::<i64>(8))]
    #[case(Buffer::alloc::<u64>(8))]
    #[case(Buffer::alloc::<f32>(8))]
    #[case(Buffer::alloc::<f64>(8))]
    fn buffer_alloc(#[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(buf) => assert_ne!(0, buf.ptr()),
        }
    }

    #[rstest]
    #[case(Buffer::alloc::<i8> (0))]
    #[case(Buffer::alloc::<u8> (0))]
    #[case(Buffer::alloc::<i16>(0))]
    #[case(Buffer::alloc::<u16>(0))]
    #[case(Buffer::alloc::<i32>(0))]
    #[case(Buffer::alloc::<u32>(0))]
    #[case(Buffer::alloc::<i64>(0))]
    #[case(Buffer::alloc::<u64>(0))]
    #[case(Buffer::alloc::<f32>(0))]
    #[case(Buffer::alloc::<f64>(0))]
    fn buffer_alloc_with_zero_length(#[case] input: Option<Buffer>) {
        if let Some(_) = input {
            panic!("input: Option<Buffer> must be None.");
        }
    }

    #[rstest]
    #[case( 1, Buffer::alloc::<i8> (1))]
    #[case( 2, Buffer::alloc::<i8> (2))]
    #[case( 3, Buffer::alloc::<i8> (3))]
    #[case( 4, Buffer::alloc::<i8> (4))]
    #[case( 8, Buffer::alloc::<i8> (8))]
    #[case( 8, Buffer::alloc::<u8> (8))]
    #[case(16, Buffer::alloc::<i16>(8))]
    #[case(16, Buffer::alloc::<u16>(8))]
    #[case(32, Buffer::alloc::<i32>(8))]
    #[case(32, Buffer::alloc::<u32>(8))]
    #[case(64, Buffer::alloc::<i64>(8))]
    #[case(64, Buffer::alloc::<u64>(8))]
    #[case(32, Buffer::alloc::<f32>(8))]
    #[case(64, Buffer::alloc::<f64>(8))]
    fn buffer_length_in_u8(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(buf) => assert_eq!(expected, buf.length::<u8>()),
        }
    }

    #[rstest]
    #[case( 0, Buffer::alloc::<i8> (1))]
    #[case( 0, Buffer::alloc::<i8> (2))]
    #[case( 0, Buffer::alloc::<i8> (3))]
    #[case( 1, Buffer::alloc::<i8> (4))]
    #[case( 2, Buffer::alloc::<i8> (8))]
    #[case( 2, Buffer::alloc::<u8> (8))]
    #[case( 4, Buffer::alloc::<i16>(8))]
    #[case( 4, Buffer::alloc::<u16>(8))]
    #[case( 8, Buffer::alloc::<i32>(8))]
    #[case( 8, Buffer::alloc::<u32>(8))]
    #[case(16, Buffer::alloc::<i64>(8))]
    #[case(16, Buffer::alloc::<u64>(8))]
    #[case( 8, Buffer::alloc::<f32>(8))]
    #[case(16, Buffer::alloc::<f64>(8))]
    fn buffer_length_in_u32(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(buf) => assert_eq!(expected, buf.length::<u32>()),
        }
    }

    #[rstest]
    #[case( 1, Buffer::alloc::<i8> (1))]
    #[case( 2, Buffer::alloc::<i8> (2))]
    #[case( 3, Buffer::alloc::<i8> (3))]
    #[case( 4, Buffer::alloc::<i8> (4))]
    #[case( 8, Buffer::alloc::<i8> (8))]
    #[case( 8, Buffer::alloc::<u8> (8))]
    #[case(16, Buffer::alloc::<i16>(8))]
    #[case(16, Buffer::alloc::<u16>(8))]
    #[case(32, Buffer::alloc::<i32>(8))]
    #[case(32, Buffer::alloc::<u32>(8))]
    #[case(64, Buffer::alloc::<i64>(8))]
    #[case(64, Buffer::alloc::<u64>(8))]
    #[case(32, Buffer::alloc::<f32>(8))]
    #[case(64, Buffer::alloc::<f64>(8))]
    fn buffer_slice_in_u8(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(buf) => {
                let slice = buf.slice::<u8>();
                assert_eq!(expected, slice.len());
                assert!(slice.iter().all(|&x| x == 0));
            }
        }
    }

    #[rstest]
    #[case( 0, Buffer::alloc::<i8> (1))]
    #[case( 0, Buffer::alloc::<i8> (2))]
    #[case( 0, Buffer::alloc::<i8> (3))]
    #[case( 1, Buffer::alloc::<i8> (4))]
    #[case( 2, Buffer::alloc::<i8> (8))]
    #[case( 2, Buffer::alloc::<u8> (8))]
    #[case( 4, Buffer::alloc::<i16>(8))]
    #[case( 4, Buffer::alloc::<u16>(8))]
    #[case( 8, Buffer::alloc::<i32>(8))]
    #[case( 8, Buffer::alloc::<u32>(8))]
    #[case(16, Buffer::alloc::<i64>(8))]
    #[case(16, Buffer::alloc::<u64>(8))]
    #[case( 8, Buffer::alloc::<f32>(8))]
    #[case(16, Buffer::alloc::<f64>(8))]
    fn buffer_slice_in_u32(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(buf) => {
                let slice = buf.slice::<u32>();
                assert_eq!(expected, slice.len());
                assert!(slice.iter().all(|&x| x == 0));
            }
        }
    }

    #[rstest]
    #[case( 1, Buffer::alloc::<i8> (1))]
    #[case( 2, Buffer::alloc::<i8> (2))]
    #[case( 3, Buffer::alloc::<i8> (3))]
    #[case( 4, Buffer::alloc::<i8> (4))]
    #[case( 8, Buffer::alloc::<i8> (8))]
    #[case( 8, Buffer::alloc::<u8> (8))]
    #[case(16, Buffer::alloc::<i16>(8))]
    #[case(16, Buffer::alloc::<u16>(8))]
    #[case(32, Buffer::alloc::<i32>(8))]
    #[case(32, Buffer::alloc::<u32>(8))]
    #[case(64, Buffer::alloc::<i64>(8))]
    #[case(64, Buffer::alloc::<u64>(8))]
    #[case(32, Buffer::alloc::<f32>(8))]
    #[case(64, Buffer::alloc::<f64>(8))]
    fn buffer_slice_mut_in_u8(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(mut buf) => {
                let slice = buf.slice_mut::<u8>();
                assert_eq!(expected, slice.len());
                assert!(slice.iter().all(|&x| x == 0));
            }
        }
    }

    #[rstest]
    #[case( 0, Buffer::alloc::<i8> (1))]
    #[case( 0, Buffer::alloc::<i8> (2))]
    #[case( 0, Buffer::alloc::<i8> (3))]
    #[case( 1, Buffer::alloc::<i8> (4))]
    #[case( 2, Buffer::alloc::<i8> (8))]
    #[case( 2, Buffer::alloc::<u8> (8))]
    #[case( 4, Buffer::alloc::<i16>(8))]
    #[case( 4, Buffer::alloc::<u16>(8))]
    #[case( 8, Buffer::alloc::<i32>(8))]
    #[case( 8, Buffer::alloc::<u32>(8))]
    #[case(16, Buffer::alloc::<i64>(8))]
    #[case(16, Buffer::alloc::<u64>(8))]
    #[case( 8, Buffer::alloc::<f32>(8))]
    #[case(16, Buffer::alloc::<f64>(8))]
    fn buffer_slice_mut_in_u32(#[case] expected: usize, #[case] input: Option<Buffer>) {
        match input {
            None => panic!("input: Option<Buffer> will be Some."),
            Some(mut buf) => {
                let slice = buf.slice_mut::<u32>();
                assert_eq!(expected, slice.len());
                assert!(slice.iter().all(|&x| x == 0));
            }
        }
    }
}
