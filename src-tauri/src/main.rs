#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::fs::File;
use std::io::prelude::*;
mod services;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

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
    let mut file = File::open("../data.json").map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    Ok(contents)
}

#[tauri::command]
fn play_mp3(file_path: &str) -> Result<(), String> {     
    let (_stream, sink) = OutputStream::try_default().unwrap();
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
    let sink = Sink::try_new(&sink).map_err(|e| e.to_string())?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sync, load, play_mp3])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
