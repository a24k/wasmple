mod console;

#[no_mangle]
pub extern "C" fn hello() {
    console::log(format!("LOG - ログだよ！"));
    console::debug(format!("DEBUG - デバッグだよ！"));
    console::info(format!("INFO - インフォメーションだよ！"));
    console::warn(format!("WARN - ワーニングだよ！"));
    console::error(format!("ERROR - エラーだよ！"));
}
