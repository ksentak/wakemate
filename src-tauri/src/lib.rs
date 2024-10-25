// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn prevent_sleep() {
    // For Windows, you could use the Windows API to prevent sleep.
    // For macOS, `caffeinate` command could be executed via Rust.
    // For Linux, it might depend on the specific desktop environment.
    println!("Sleep prevention activated.");
    // Platform-specific sleep prevention code goes here
}

#[tauri::command]
fn allow_sleep() {
    println!("Sleep prevention deactivated.");
    // Code to revert sleep prevention
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![prevent_sleep, allow_sleep])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
