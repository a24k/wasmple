mod string;

use super::{BufferManager, BufferPtr};

pub trait BufferConverter<T> {
    fn from(ptr: BufferPtr) -> Option<T>;
    fn into(&self) -> Option<BufferPtr>;
}

pub fn into<T>(ptr: BufferPtr) -> Option<T>
where
    T: BufferConverter<T>,
{
    T::from(ptr)
}

pub fn from<T>(obj: T) -> Option<BufferPtr>
where
    T: BufferConverter<T>,
{
    T::into(&obj)
}
