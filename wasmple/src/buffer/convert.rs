mod string;

use super::{BufferManager, BufferPtr};

pub mod into {
    use super::BufferPtr;

    use super::string;

    pub fn string(ptr: BufferPtr) -> Option<String> {
        string::from(ptr)
    }
}

pub mod from {
    use super::BufferPtr;

    use super::string;

    pub fn string(str: String) -> BufferPtr {
        string::into(str)
    }
}
