// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



#[tauri::command]
fn send_notification(title: String, body: String) -> Result<(), String> {
    // In Tauri 2.x, notifications are handled differently
    println!("Notification: {} - {}", title, body);
    Ok(())
}

#[tauri::command]
fn vibrate() -> Result<(), String> {
    // Vibration is handled by the frontend JavaScript
    // This command exists for potential future native vibration support
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![send_notification, vibrate])
        .setup(|_app| {
            #[cfg(target_os = "android")]
            {
                let main_window = app.get_webview_window("main").unwrap();
                let _ = main_window.set_fullscreen(true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}