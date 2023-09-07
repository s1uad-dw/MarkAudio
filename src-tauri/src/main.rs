#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::fs::File;
use std::io::prelude::*;
mod services;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn sync(ip:String, username:String, password:String, shop_id:String, marketing_interval:String) -> Result<String, String>{
    print!("sync");
    let sync_thread = thread::spawn(|| -> Result<(), String> {
        let mut server: services::Server = services::Server::new(ip,username,password);
        server.sync_dirs("./Music", "../music")?;
        server.sync_dirs("./Marketing", "../marketing")?;
        let mut data: services::Data = services::Data::new(server.ip, server.username, server.password, shop_id, marketing_interval);
        data.save()?;
        Ok(())
    });
    match sync_thread.join(){
        Ok(result) => {
            if let Err(e) = result {
                Err(format!("Во время синхронизации файлов произошла ошибка: {:?}", e))
            } else {
                Ok("Синхронизация файлов успешно завершена.".to_string())
            }
        },
        Err(e) => Err(format!("Во время синхронизации файлов произошла ошибка: {:?}", e)),
    }
}

#[tauri::command]
fn load() -> Result<String, String> {
    let mut file = match File::open("../data.json") {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create("../data.json").map_err(|e| e.to_string())?;
            new_file.write_all(b"{}").map_err(|e| e.to_string())?;
            File::open("../data.json").map_err(|e| e.to_string())?
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    Ok(contents)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sync, load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
