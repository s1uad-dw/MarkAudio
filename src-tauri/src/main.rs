#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use chrono::{DateTime, Utc, FixedOffset, Duration};
mod data;
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
    marketing_interval: String
) -> Result<(), String> {
    let data = data::Data::new(
        ip,
        username,
        password,
        shop_id,
        marketing_interval
    );
    data.save()
}

#[tauri::command]
fn get_local_files() -> Result<String, String> {
    let dirs: [&str;2] = ["./music", "./marketing"];
    let mut filenames:Vec<Vec<String>> = Vec::new();
    for dir in dirs{
        match services::take_local_dir_files(dir){
            Ok(local_files) => filenames.push(local_files),
            Err(err) => return Err(err.into())
        }
    }
    match serde_json::to_string(&filenames){
        Ok(json_str) => Ok(json_str),
        Err(err) => Err(err.to_string())
    }
}

// #[tauri::command]
// fn write_time() -> Result<(), String>{
//     match data::Data::load_data() {
//         Ok(mut data) => {
//             data.start_playing_time = Some(Utc::now().with_timezone(&FixedOffset::east(3 * 3600)).to_string());
//             data.save()?;
//             Ok(())
//         }
//         Err(err) => Err(err.to_string())
//     }
// }

// #[tauri::command]
// fn get_time_difference() -> Result<i32, String>{
//     match data::Data::load_data() {
//         Ok(data) => {
//             let start_time_str = match data.start_playing_time{
//                 Some(time) => time,
//                 None => return Err("Ошибка при считывании времени старта проигрывания".to_string()),
//             };
//             let current_time: DateTime<Utc> = Utc::now();
//             match DateTime::parse_from_str(&start_time_str, "%Y-%m-%d %H:%M:%S%.f %z"){
//                 Ok(start_time) => {
//                     let start_time = start_time.with_timezone(&Utc);
//                     println!("{}", start_time.to_string());
//                     let duration: Duration = current_time.signed_duration_since(start_time);
//                     match Some(duration.num_minutes() as i32){
//                         Some(diff) => return Ok(diff),
//                         None => return Err("Ошибка при считывании времени старта проигрывания".to_string()),
//                     }
//                 },
//                 Err(err) => return Err(format!("Ошибка при считывании времени старта проигрывания{}", err)),
//             }
//         },
//         Err(err) => return Err(format!("Ошибка при считывании времени старта проигрывания{}", err)),
//     }
// }


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_sources,
            load,
            save,
            get_local_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
