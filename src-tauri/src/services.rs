use serde::{Deserialize, Serialize};
use serde_json;
use ssh2::Session;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

pub struct Server {
    pub ip: String,
    pub username: String,
    pub password: String,
}

impl Server {
    pub fn new(ip: String, username: String, password: String) -> Server {
        Server {
            ip: ip,
            username: username,
            password: password,
        }
    }

    pub fn download_files(
        &mut self,
        remote_dir: &str,
        tracks: Vec<&str>,
        local_dir: &str,
    ) -> Result<(), String> {
        let tcp = TcpStream::connect(format!("{}:22", self.ip))
            .map_err(|e| format!("Ошибка соединения TCP: {:?}", e))?;
        let mut sess = Session::new().map_err(|e| format!("Ошибка создания сессии: {:?}", e))?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| format!("Ошибка рукопожатия: {:?}", e))?;
        sess.userauth_password(&self.username, &self.password)
            .map_err(|e| format!("Ошибка аутентификации: {:?}", e))?;

        for index in 0..tracks.len() {
            let (mut remote_file, _stat) = sess
                .scp_recv(Path::new(&format!("{}/{}", remote_dir, tracks[index])))
                .map_err(|e| format!("Ошибка приема SCP: {:?}", e))?;
            let mut contents = Vec::new();
            let mut file = File::create(format!("{}/{}", local_dir, tracks[index]))
                .map_err(|e| format!("Ошибка создания файла: {:?}", e))?;
            remote_file
                .read_to_end(&mut contents)
                .map_err(|e| format!("Ошибка чтения файла: {:?}", e))?;
            file.write_all(&contents)
                .map_err(|e| format!("Ошибка записи файла: {:?}", e))?;
        }

        Ok(())
    }

    pub fn ls(&mut self, dir: Option<String>) -> Result<String, String> {
        let tcp = TcpStream::connect(format!("{}:22", self.ip))
            .map_err(|e| format!("Ошибка соединения TCP: {:?}", e))?;
        let mut sess = Session::new().map_err(|e| format!("Ошибка создания сессии: {:?}", e))?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| format!("Ошибка рукопожатия: {:?}", e))?;
        sess.userauth_password(&self.username, &self.password)
            .map_err(|e| format!("Ошибка аутентификации: {:?}", e))?;
        let mut channel = sess
            .channel_session()
            .map_err(|e| format!("Ошибка создания канала: {:?}", e))?;
        match dir {
            Some(value) => channel
                .exec(&format!("ls {}", value))
                .map_err(|e| format!("Ошибка выполнения команды ls: {:?}", e))?,
            None => channel
                .exec("ls")
                .map_err(|e| format!("Ошибка выполнения команды ls: {:?}", e))?,
        }
        let mut s = String::new();
        channel
            .read_to_string(&mut s)
            .map_err(|e| format!("Ошибка чтения канала: {:?}", e))?;
        channel
            .wait_close()
            .map_err(|e| format!("Ошибка закрытия канала: {:?}", e))?;
        Ok(s)
    }

    pub fn take_local_dir_filenames(&mut self, local_dir: &str) -> Vec<String> {
        //take filenames from local dir
        let paths = fs::read_dir(local_dir).unwrap();
        let mut local_filenames = Vec::new();
        for path in paths {
            let file_name = String::from(path.unwrap().file_name().to_string_lossy());
            local_filenames.push(file_name);
        }
        local_filenames
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    ip: String,
    username: String,
    password: String,
    shop_id: String,
    marketing_interval: String,
}

impl Data {
    pub fn new(
        ip: String,
        username: String,
        password: String,
        shop_id: String,
        marketing_interval: String,
    ) -> Data {
        Data {
            ip: ip,
            username: username,
            password: password,
            shop_id: shop_id,
            marketing_interval: marketing_interval,
        }
    }
    pub fn save(&mut self) -> Result<(), String> {
        let json_string =
            serde_json::to_string(&self).map_err(|e| format!("Ошибка сериализации: {:?}", e))?;
        let mut file =
            File::create("./data.json").map_err(|e| format!("Ошибка создания файла: {:?}", e))?;
        file.write_all(json_string.as_bytes())
            .map_err(|e| format!("Ошибка записи в файл: {:?}", e))?;
        Ok(())
    }
}

pub fn create_if_not_exists(path_type: PathType, path: &str,) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("Файл уже существует")
            } else if metadata.is_dir() {
                println!("Директория уже существует")
            }
            Ok(())
        }
        Err(_) => match path_type {
            PathType::Dir => match fs::create_dir(path) {
                Ok(_) => {
                    println!("Создание директории завершено успешно");
                    Ok(())
                }
                Err(err) => Err(format!("При создании директории {} произошла ошибка: {}", path.split("/").last().unwrap(), err)),
            },
            PathType::File => match File::create(path) {
                Ok(_) => {
                    println!("Создание файла завершено успешно");
                    Ok(())
                }
                Err(err) => Err(format!("При создании файла {} произошла ошибка: {}", path.split("/").last().unwrap(), err)),
            },
        },
    }
}

pub enum PathType {
    File,
    Dir,
}
