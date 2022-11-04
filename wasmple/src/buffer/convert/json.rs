use serde_json::Value;

use super::{BufferConverter, BufferPtr};

impl BufferConverter<Value> for Value {
    fn from(ptr: BufferPtr) -> Option<Value> {
        serde_json::from_str(&super::into::<String>(ptr)?).ok()
    }

    fn into(&self) -> Option<BufferPtr> {
        super::from(serde_json::to_string(self).ok()?)
    }
}
