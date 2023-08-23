// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        // 新增关闭提示的逻辑
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    api.prevent_close();

                    let window = event.window().clone();
                    tauri::api::dialog::confirm(
                        Some(&event.window()),
                        "关闭应用",
                        "确定关闭?",
                        move |answer| {
                            if answer {
                                let _ = window.close();
                            }
                        },
                    )
                }
                _ => {} //todo
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
