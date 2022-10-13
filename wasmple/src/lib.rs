mod console;

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn hello() {
    panic!("OMG - あたふた・・・");
    console::log(format!("LOG - ログだよ！"));
    console::debug(format!("DEBUG - デバッグだよ！"));
    console::info(format!("INFO - インフォメーションだよ！"));
    console::warn(format!("WARN - ワーニングだよ！"));
    console::error(format!("ERROR - エラーだよ！"));
}
