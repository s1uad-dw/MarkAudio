use rand::Rng;
use ssh2::{Channel, Session};
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

    pub fn connect(&self) -> Result<(Session, Channel), String> {
        let mut sess = match Session::new() {
            Ok(sess) => sess,
            Err(err) => return Err(format!("Ошибка создания ssh сессии {}", err)),
        };
        match TcpStream::connect(format!("{}:22", &self.ip)) {
            Ok(tcp) => {
                sess.set_tcp_stream(tcp);
                match sess.handshake() {
                    Ok(_) => match sess.userauth_password(&self.username, &self.password) {
                        Ok(_) => match sess.channel_session() {
                            Ok(channel) => return Ok((sess, channel)),
                            Err(err) => return Err(format!("Ошибка создания канала: {}", err)),
                        },
                        Err(err) => return Err(format!("Ошибка подключения к серверу 1 {}", err)),
                    },
                    Err(err) => return Err(format!("Ошибка подключения к серверу 2 {}", err)),
                }
            }
            Err(err) => return Err(format!("Ошибка подключения к серверу 3 {}", err)),
        }
    }

    pub fn download_missing_tracks(&self, quantity: i32) -> Result<(), String> {
        match self.connect() {
            Ok((session, mut channel)) => {
                channel
                    .exec("cd Music\n clear \n ls")
                    .map_err(|e| format!("1{}", e))?;
                let mut answer = String::new();
                channel
                    .read_to_string(&mut answer)
                    .map_err(|e| e.to_string())?;
                let mut music: Vec<&str> = answer.split('\n').collect();
                let mut rng = rand::thread_rng();
                // let need_len = music.len() + quantity as usize;
                let mut counter: i32 = 0;
                while counter < quantity {
                    let random_munber = rng.gen_range(1..=music.len() - 1);
                    let current_file = music[random_munber];
                    match session.scp_recv(Path::new(&format!("Music/{}", &current_file))) {
                        Ok((mut remote_file, _stat)) => {
                            let mut contents = Vec::new();
                            remote_file.read_to_end(&mut contents).map_err(|e| {
                                format!("Ошибка чтения удаленного файла: {}", e)
                            })?;
                            std::fs::write(format!("music/{}", &current_file.replace(" ", "").replace("(", "").replace(")", "")), &contents)
                                .map_err(|e| {
                                    format!("Ошибка сохранения удаленного файла: {}", e)
                                })?;
                            music.remove(random_munber);
                            counter +=1;
                        }
                        Err(_) => {
                            // counter -=1;
                            println!("abababbababababab")
                        },
                    }
                }
                return Ok(());
            }
            Err(err) => return Err(err),
        };
    }
}
