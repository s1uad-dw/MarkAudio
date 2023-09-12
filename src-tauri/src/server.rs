use ssh2::Session;
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

    pub fn connect(self) -> Result<Session, String>{
        let mut sess = match Session::new(){
            Ok(sess) => sess,
            Err(err) => Err(format!("Оибка создания ssh сессии {}", err))
        };
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

    pub fn download_missing_tracks(
        &self,
        ip: String,
        username: String,
        password: String,
        quantity: i32,
    ) -> Result<(), String> {

    }
}
