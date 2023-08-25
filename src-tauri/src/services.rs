use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use ssh2::Session;
use std::fs::File;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

pub struct Server{
    pub ip: String,
    pub username: String,
    pub password: String
}

impl Server{
    pub fn new(ip:String, username:String, password:String) -> Server{
        Server {
            ip: ip,
            username: username,
            password: password,
        }
    }

    pub fn download_files(&mut self, remote_dir: &str, tracks: Vec<&str>, local_dir: &str) -> Result<(), String> {
        let tcp = TcpStream::connect(format!("{}:22", self.ip)).map_err(|e| format!("Ошибка соединения TCP: {:?}", e))?;
        let mut sess = Session::new().map_err(|e| format!("Ошибка создания сессии: {:?}", e))?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err(|e| format!("Ошибка рукопожатия: {:?}", e))?;
        sess.userauth_password(&self.username, &self.password).map_err(|e| format!("Ошибка аутентификации: {:?}", e))?;
    
        for index in 0..tracks.len() {
            let (mut remote_file, _stat) = sess.scp_recv(Path::new(&format!("{}/{}", remote_dir, tracks[index]))).map_err(|e| format!("Ошибка приема SCP: {:?}", e))?;
            let mut contents = Vec::new();
            let mut file = File::create(format!("{}/{}", local_dir, tracks[index])).map_err(|e| format!("Ошибка создания файла: {:?}", e))?;
            remote_file.read_to_end(&mut contents).map_err(|e| format!("Ошибка чтения файла: {:?}", e))?;
            file.write_all(&contents).map_err(|e| format!("Ошибка записи файла: {:?}", e))?;
        }
    
        Ok(())
    }

    pub fn ls(&mut self, dir: Option<String>) -> Result<String, String> {
        let tcp = TcpStream::connect(format!("{}:22", self.ip)).map_err(|e| format!("Ошибка соединения TCP: {:?}", e))?;
        let mut sess = Session::new().map_err(|e| format!("Ошибка создания сессии: {:?}", e))?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err(|e| format!("Ошибка рукопожатия: {:?}", e))?;
        sess.userauth_password(&self.username, &self.password).map_err(|e| format!("Ошибка аутентификации: {:?}", e))?;
        let mut channel = sess.channel_session().map_err(|e| format!("Ошибка создания канала: {:?}", e))?;
        match dir {
            Some(value) => channel.exec(&format!("ls {}", value)).map_err(|e| format!("Ошибка выполнения команды ls: {:?}", e))?,
            None => channel.exec("ls").map_err(|e| format!("Ошибка выполнения команды ls: {:?}", e))?
        }
        let mut s = String::new();
        channel.read_to_string(&mut s).map_err(|e| format!("Ошибка чтения канала: {:?}", e))?;
        channel.wait_close().map_err(|e| format!("Ошибка закрытия канала: {:?}", e))?;
        Ok(s)
    }

    pub fn take_local_dir_filenames(&mut self, local_dir:&str) -> Vec<String>{
        //take filenames from local dir
        let paths = fs::read_dir(local_dir).unwrap();
        let mut local_filenames = Vec::new();
        for path in paths {
            let file_name = String::from(path.unwrap().file_name().to_string_lossy());
            local_filenames.push(file_name);
        }
        local_filenames
    }

    pub fn sync_dirs(&mut self, remote_dir:&str, local_dir:&str) -> Result<(), String> {
        //take filenames from remote dir
        let answer: String = self.ls(Some(remote_dir.to_string()))?;
        let _clean_str: String = String::new();
        let remote_filenames: Vec<&str> = answer.trim().split("\n").collect();
    
        //take filenames from local dir
        let local_filenames = self.take_local_dir_filenames(local_dir);
    
        //remove extra files
        let mut extra_files = Vec::new();
        for file in local_filenames{
            if !remote_filenames.contains(&file.as_str()){
                extra_files.push(file);
            }
        }
        for path in extra_files {
            fs::remove_file(format!("{local_dir}/{path}")).map_err(|e| format!("Ошибка удаления файла: {:?}", e))?;
        }
    
        //download missing files
        let local_filenames = self.take_local_dir_filenames(local_dir);
        let mut missing_files = Vec::new();
        for file in remote_filenames{
            if !local_filenames.contains(&file.to_string()){
                missing_files.push(file);
            }
        }
        self.download_files(remote_dir, missing_files, local_dir)?;
    
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data{
    ip: String,
    username: String,
    password: String,
    shop_id: String,
    marketing_interval: String
}

impl Data{
    pub fn new(ip: String,username: String,password: String,shop_id: String,marketing_interval: String) -> Data{
        Data {
            ip: ip,
            username: username,
            password: password,
            shop_id: shop_id,
            marketing_interval: marketing_interval
        }
    }
    pub fn save(&mut self) -> Result<(), String> {
        let json_string = serde_json::to_string(&self).map_err(|e| format!("Ошибка сериализации: {:?}", e))?;
        let mut file = File::create("../data.json").map_err(|e| format!("Ошибка создания файла: {:?}", e))?;
        file.write_all(json_string.as_bytes()).map_err(|e| format!("Ошибка записи в файл: {:?}", e))?;
        Ok(())
    }
}