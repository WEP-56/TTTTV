use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandChild;

struct AppState {
    backend_process: Mutex<Option<CommandChild>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .manage(AppState {
            backend_process: Mutex::new(None),
        })
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.webview_windows().values().next() {
                    window.open_devtools();
                }
            }

            let app_handle = app.app_handle().clone();
            
            tauri::async_runtime::spawn(async move {
                match app_handle.shell().sidecar("moovie") {
                    Ok(command) => {
                        match command.spawn() {
                            Ok((_rx, child)) => {
                                let state = app_handle.state::<AppState>();
                                let mut backend = state.backend_process.lock().unwrap();
                                *backend = Some(child);
                                println!("Backend sidecar started successfully");
                            }
                            Err(e) => eprintln!("Failed to spawn backend sidecar: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to create sidecar command: {}", e),
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let app_handle = window.app_handle();
                let state = app_handle.state::<AppState>();
                let mut backend = state.backend_process.lock().unwrap();
                if let Some(child) = backend.take() {
                    let _ = child.kill();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
