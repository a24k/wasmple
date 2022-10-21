use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;

static MAP: Lazy<Mutex<HashMap<u32, Union>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn dump<T: Debug>(value: &T) {
    println!("[dump] {:?}", value);
    println!("       as {}", std::any::type_name::<T>());
}

#[derive(Debug)]
struct Generic<T>
where
    T: Debug,
{
    value: T,
}

#[derive(Debug)]
enum Union<'a> {
    Int(Generic<i32>),
    Str(Generic<&'a str>),
    UInt(Generic<u32>),
}

impl<T> Drop for Generic<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        dump(self);
    }
}

fn main() {
    let mut map = MAP.lock().unwrap();

    println!("[main] insert 3 items");
    map.insert(1, Union::Int(Generic { value: 11 }));
    map.insert(2, Union::Str(Generic { value: "22" }));
    map.insert(3, Union::UInt(Generic { value: 33 }));
    dump(&map);

    println!("[main] clear all items");
    map.clear();

    println!("[main] finish");
    dump(&map);
}
