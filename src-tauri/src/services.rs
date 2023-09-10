use std::fs;
use std::fs::File;

pub fn create_if_not_exists(path_type: PathType, path: &str) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(_) => match path_type {
            PathType::Dir => match fs::create_dir(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!(
                    "При создании директории {} произошла ошибка: {}",
                    path.split("/").last().unwrap(),
                    err
                )),
            },
            PathType::File => match File::create(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!(
                    "При создании файла {} произошла ошибка: {}",
                    path.split("/").last().unwrap(),
                    err
                )),
            },
        },
    }
}

pub fn take_local_dir_files(local_dir: &str) -> Result<Vec<String>, String> {
    match fs::read_dir(local_dir) {
        Ok(files) => {
            let mut local_filenames: Vec<String> = Vec::new();
            for file in files {
                match file {
                    Ok(file) => {
                        let file_name = String::from(file.file_name().to_string_lossy());
                        local_filenames.push(file_name);
                    },
                    Err(err) => return Err(format!("При прочтени директории произошла ошибка: {}", err))
                }
            }
            Ok(local_filenames)
        }
        Err(err) => Err(format!("При прочтени директории произошла ошибка: {}", err))
    }
}

pub enum PathType {
    File,
    Dir,
}
