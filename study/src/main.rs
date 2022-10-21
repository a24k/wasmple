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
enum Union {
    I32(Generic<i32>),
    STR(Generic<String>),
    U32(Generic<u32>),
}

#[derive(Debug)]
enum ValueUnion<'a> {
    I32(&'a i32),
    STR(&'a String),
    U32(&'a u32),
}

impl Union {
    fn get(&self) -> ValueUnion {
        match self {
            Union::I32(gen) => ValueUnion::I32(gen.get()),
            Union::STR(gen) => ValueUnion::STR(gen.get()),
            Union::U32(gen) => ValueUnion::U32(gen.get()),
        }
    }
}

#[derive(Debug)]
struct Generic<T>
where
    T: Debug,
{
    value: T,
}

impl<T: Debug> Generic<T> {
    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let mut map = MAP.lock().unwrap();

    println!("[main] insert 3 items -----------------------------------");
    map.insert(1, Union::I32(Generic { value: 11 }));
    map.insert(
        2,
        Union::STR(Generic {
            value: String::from("22"),
        }),
    );
    map.insert(3, Union::U32(Generic { value: 33 }));
    dump(&map);

    println!("[main] get item: 1 --------------------------------------");
    let uni = map.get(&2).unwrap();
    dump(uni);
    let v = uni.get();
    dump(&v);

    println!("[main] get item: 2 --------------------------------------");
    let uni = map.get(&2).unwrap();
    dump(uni);
    let v = uni.get();
    dump(&v);

    println!("[main] finish -------------------------------------------");
}
