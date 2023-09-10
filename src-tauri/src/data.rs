use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    ip: String,
    username: String,
    password: String,
    shop_id: String,
    marketing_interval: String,
    recent_tracks: Option<[String; 10]>,
    pub start_playing_time: Option<String>
}

impl Data {
    pub fn new(
        ip: String,
        username: String,
        password: String,
        shop_id: String,
        marketing_interval: String,
        recent_tracks: Option<[String; 10]>,
        start_playing_time: Option<String>
    ) -> Data {
        Data {
            ip: ip,
            username: username,
            password: password,
            shop_id: shop_id,
            marketing_interval: marketing_interval,
            recent_tracks: recent_tracks,
            start_playing_time: start_playing_time
        }
    }

    pub fn load_data() -> Result<Data, String>{
        let file = match File::open("data.json"){
            Ok(file) => {
                let reader = BufReader::new(file);
                let data: Data = match serde_json::from_reader(reader){
                    Ok(data) => data,
                    Err(_) => Data::default()
                };
                Ok(data)
            }
            Err(err) => Err(format!("При открытии файла произошла ошибка: {}", err.to_string()))
        };
        file
    }
    pub fn save(&self) -> Result<(), String>{
        let json_str:String = match serde_json::to_string(self){
            Ok(json_str) => json_str,
            Err(err) => return Err(err.to_string())
        };
        let mut json_file = match File::create("./data.json"){
            Ok(file) => file,
            Err(err) => return Err(err.to_string())
        };
        match json_file.write_all(json_str.as_bytes()){
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}
