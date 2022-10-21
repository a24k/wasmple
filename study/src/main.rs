use std::alloc::Layout;
use std::slice;

fn typename<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn alloc<T>(length: usize)
where
    T: std::fmt::Debug,
{
    let align = std::mem::align_of::<T>();
    let unit = std::mem::size_of::<T>();
    println!(
        "  [alloc] length = {}, align = {}, unit = {}",
        length, align, unit
    );

    let layout = Layout::from_size_align(length * unit, align).unwrap();
    println!("  [alloc] layout = {:?}", layout);

    let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
    println!("  [alloc] ptr = {:?}, is_null = {}", ptr, ptr.is_null());

    let slice = unsafe { slice::from_raw_parts_mut(ptr as *mut T, length) };
    println!("  [alloc] slice = {:?} as {}", &slice, typename(&slice));

    unsafe {
        std::alloc::dealloc(ptr, layout);
    }
}

fn main() {
    println!("[main] call alloc<u16>(0)");
    alloc::<u16>(0);

    println!("[main] call alloc<u16>(8)");
    alloc::<u16>(8);

    println!("[main] call alloc<u16>(MAX)");
    alloc::<u16>(usize::MAX / std::mem::size_of::<u16>());
}
