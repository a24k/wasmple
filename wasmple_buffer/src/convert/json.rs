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

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::super::{from, into, BufferManager, JsonConvertee};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[rstest]
    #[case(json!({}))]
    #[case(json!({"a": "b", "c": "d"}))]
    #[case(json!({"x": 0, "y": 1}))]
    fn convert_json_untyped(#[case] input: Value) {
        let ptr = from(input.clone());

        match ptr {
            None => panic!("ptr: Option<BufferPtr> will be Some."),
            Some(ptr) => {
                let output = into::<Value>(ptr);

                match output {
                    None => panic!("ptr: Option<String> will be Some."),
                    Some(output) => {
                        assert_eq!(input, output);
                    }
                }

                BufferManager::lock().dealloc(ptr);
            }
        }
    }

    #[derive(Serialize, Deserialize, JsonConvertee, Debug, PartialEq, Eq, Clone)]
    struct TypedJson {
        name: String,
        age: usize,
    }

    #[rstest]
    #[case(TypedJson{name: String::from("James"), age: 25})]
    #[case(TypedJson{name: String::from("太郎"), age: 35})]
    fn convert_json_typed(#[case] input: TypedJson) {
        let ptr = from(input.clone());

        match ptr {
            None => panic!("ptr: Option<BufferPtr> will be Some."),
            Some(ptr) => {
                let output = into::<TypedJson>(ptr);

                match output {
                    None => panic!("ptr: Option<String> will be Some."),
                    Some(output) => {
                        assert_eq!(input, output);
                    }
                }

                BufferManager::lock().dealloc(ptr);
            }
        }
    }
}
