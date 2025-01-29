use crate::capture::ScreenRecorder;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static RECORDER: Lazy<Mutex<ScreenRecorder>> = Lazy::new(|| {
    Mutex::new(ScreenRecorder::new())
});

#[tauri::command]
pub fn greet(name: String) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
pub fn record(is_recording: bool) -> Result<bool, String> {
    let recorder = RECORDER.lock().unwrap();
    
    if is_recording {
        recorder.stop_recording()?;
        Ok(false)
    } else {
        recorder.start_recording()?;
        Ok(true)
    }
}

#[tauri::command]
pub fn get_recording_status() -> bool {
    let recorder = RECORDER.lock().unwrap();
    recorder.is_recording()
}
