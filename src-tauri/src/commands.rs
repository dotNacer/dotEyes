#[tauri::command]
pub fn greet(name: String) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
pub fn record(is_recording: bool) -> bool {
    println!("Recording state changed to: {}", !is_recording);
    !is_recording
}
