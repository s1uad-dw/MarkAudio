#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use services::create_if_not_exists;
mod services;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn check_sources() -> Result<(), String> {
    let sources: [(services::PathType, &str); 3] = [
        (services::PathType::Dir, "./music"),
        (services::PathType::Dir, "./marketing"),
        (services::PathType::File, "./data.json"),
    ];
    for source in sources {
        match create_if_not_exists(source.0, source.1) {
            Ok(()) => (),
            Err(err) => return Err(err.into()),
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_sources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
