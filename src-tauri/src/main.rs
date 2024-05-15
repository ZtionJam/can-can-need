#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::thread;

use tauri::Manager;

use actions::*;

mod actions;

mod config;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            thread::spawn(move || {
                link_start(&window);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet,set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
