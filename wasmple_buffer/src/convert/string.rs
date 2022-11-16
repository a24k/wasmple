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

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::super::{from, into};

    #[rstest]
    #[case("abc123")]
    #[case("日本語でこんにちわ")]
    #[case("😁😊😋")]
    fn convert_string(#[case] input: String) {
        let ptr = from(input.clone());

        match ptr {
            None => panic!("ptr: Option<BufferPtr> will be Some."),
            Some(ptr) => {
                let output = into::<String>(ptr);

                match output {
                    None => panic!("ptr: Option<String> will be Some."),
                    Some(output) => {
                        assert_eq!(input, output);
                    }
                }
            }
        }
    }
}
