mod commands;
mod capture;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_shell::init()) // uncomment to use shell
        .invoke_handler(tauri::generate_handler![greet, record, get_recording_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
