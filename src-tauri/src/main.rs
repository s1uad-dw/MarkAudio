#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::fs::File;
use std::io::prelude::*;
mod services;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
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
fn abab(){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
    sink.append(source);
    let source = SineWave::open()

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sync, load, abab])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
