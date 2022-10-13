extern "C" {
    fn console_log();
}

#[no_mangle]
pub extern "C" fn hello() {
    unsafe {
        console_log();
    }
}
