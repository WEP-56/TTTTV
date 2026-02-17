use std::process::{Command, Stdio};
use std::sync::Mutex;
use tauri::Manager;

struct AppState {
    backend_process: Mutex<Option<std::process::Child>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
                let exe_path = std::env::current_exe().ok();
                let exe_dir = exe_path.and_then(|p| p.parent().map(|p| p.to_path_buf()));
                
                let backend_exe = if let Some(dir) = exe_dir {
                    let candidate = dir.join("ttttv.exe");
                    if candidate.exists() {
                        candidate
                    } else {
                        dir.join("moovie.exe")
                    }
                } else {
                    std::path::PathBuf::from("ttttv.exe")
                };

                if backend_exe.exists() {
                    let config_dir = backend_exe.parent().unwrap_or_else(|| std::path::Path::new("."));
                    let working_dir = if config_dir.join("config").exists() {
                        config_dir.to_path_buf()
                    } else {
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
                    };

                    match Command::new(&backend_exe)
                        .current_dir(&working_dir)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                    {
                        Ok(child) => {
                            let state = app_handle.state::<AppState>();
                            let mut backend = state.backend_process.lock().unwrap();
                            *backend = Some(child);
                        }
                        Err(e) => {
                            eprintln!("Failed to start backend: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let app_handle = window.app_handle();
                let state = app_handle.state::<AppState>();
                let mut backend = state.backend_process.lock().unwrap();
                if let Some(mut child) = backend.take() {
                    let _ = child.kill();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
