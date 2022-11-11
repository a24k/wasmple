use super::{BufferConverter, BufferPtr};

pub trait JsonConvertee: serde::Serialize + serde::de::DeserializeOwned {}

impl JsonConvertee for serde_json::Value {}

impl<T> BufferConverter<T> for T
where
    T: JsonConvertee,
{
    fn from(ptr: BufferPtr) -> Option<T> {
        serde_json::from_str(&super::into::<String>(ptr)?).ok()
    }

    fn into(&self) -> Option<BufferPtr> {
        super::from(serde_json::to_string(self).ok()?)
    }
}
