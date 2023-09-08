use std::fs;
use std::fs::File;

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
