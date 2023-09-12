#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use chrono::{DateTime, Utc, FixedOffset, Duration};
mod data;
mod services;
mod server;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn check_sources() -> Result<(), String> {
    let sources: [(services::PathType, &str); 3] = [
        (services::PathType::Dir, "./music"),
        (services::PathType::Dir, "./marketing"),
        (services::PathType::File, "./data.json"),
    ];
    for source in sources {
        match services::create_if_not_exists(source.0, source.1) {
            Ok(()) => (),
            Err(err) => return Err(err.into()),
        }
    }
    Ok(())
}

#[tauri::command]
fn load() -> Result<String, String> {
    let data = match data::Data::load_data() {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };
    let json_str: String = match serde_json::to_string(&data) {
        Ok(json_str) => json_str.into(),
        Err(err) => {
            return Err(format!(
                "При считывании данных из файла произошла ошибка: {}",
                err.to_string()
            ))
        }
    };
    Ok(json_str)
}

#[tauri::command]
fn save(
    ip: String,
    username: String,
    password: String,
    shop_id: String,
    marketing_interval: String,
) -> Result<(), String> {
    let data = data::Data::new(ip, username, password, shop_id, marketing_interval);
    data.save()
}

#[tauri::command]
fn get_local_files() -> Result<String, String> {
    let dirs: [&str; 2] = ["./music", "./marketing"];
    let mut filenames: Vec<Vec<String>> = Vec::new();
    for dir in dirs {
        match services::take_local_dir_files(dir) {
            Ok(local_files) => filenames.push(local_files),
            Err(err) => return Err(err.into()),
        }
    }
    match serde_json::to_string(&filenames) {
        Ok(json_str) => Ok(json_str),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn remove_file(path: String) -> Result<(), String> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Во время удаления файла произошла ошибка: {}", err)),
    }
}

#[tauri::command]
fn download_missing_tracks(
    ip: String,
    username: String,
    password: String,
    quantity: i32,
) -> Result<(), String> {
    let server = server::Server::new(ip, username, password);
    server.download_files(remote_dir, tracks, local_dir)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_sources,
            load,
            save,
            get_local_files,
            remove_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
