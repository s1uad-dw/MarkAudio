#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use services::create_if_not_exists;
mod services;
mod data;

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
#[tauri::command]
fn load() -> Result<String, String> {
    let data = match data::Data::load_data(){
        Ok(data) => data,
        Err(err) => return Err(err.to_string())
    };
    let json_str:String = match serde_json::to_string(&data){
        Ok(json_str) => json_str,
        Err(err) => return Err(format!("При считывании данных из файла произошла ошибка: {}", err.to_string()))
    };
    Ok(json_str)
}

// #[tauri::command]
// fn play() -> Result<(), String> {
//     let sources: [(services::PathType, &str); 3] = [
//         (services::PathType::Dir, "./music"),
//         (services::PathType::Dir, "./marketing"),
//         (services::PathType::File, "./data.json"),
//     ];
//     for source in sources {
//         match create_if_not_exists(source.0, source.1) {
//             Ok(()) => (),
//             Err(err) => return Err(err.into()),
//             }
//         }
//     Ok(())
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_sources, load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
