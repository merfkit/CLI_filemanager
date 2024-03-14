use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

const ERROR_MSG_LINE_READ: &str = "Ошибка чтения строки";
const ERROR_MSG_LINE_DIR: &str = "Ошибка. Указанная директория не существует";
const ERROR_MSG_COPY_FILE: &str = "Ошибка копирования файла";
const ERROR_MSG_FILE_DELETE: &str = "Ошибка удаления файла";
const ERROR_MSG_FOLDER_CREATE: &str = "Ошибка создания директории";

fn main() {
    let directory_path = read_directory();
    let mut mask_list = read_mask();
    process_files(&directory_path, &mask_list);
}
fn process_files(directory_path: &Path, mask_list: &HashSet<String>) {
    //идем по всем файлам в указанной директории
    for entry in fs::read_dir(directory_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut file_processed = false;

        if path.is_file() {
            //получаем имя файла
            let filename = path.file_name().and_then(std::ffi::OsStr::to_str).unwrap();

            for mask in mask_list {
                if filename.contains(mask) {
                    let folder_path = directory_path.join(mask);
                    if !folder_path.exists() {
                        fs::create_dir(&folder_path).expect(ERROR_MSG_FOLDER_CREATE);
                    }
                    //копируем файл в соответствующую директорию
                    let dest_path = folder_path.join(path.file_name().unwrap());
                    fs::copy(&path, &dest_path).expect(ERROR_MSG_COPY_FILE);
                    fs::remove_file(&path).expect(ERROR_MSG_FILE_DELETE);
                    file_processed = true;
                }
            }
            //в имени файла не нашлось ни 1 маски
            if file_processed == false {
                if let Some(extension) = path.extension().and_then(std::ffi::OsStr::to_str) {
                    let folder_path = directory_path.join(extension);
                    if !folder_path.exists() {
                        fs::create_dir(&folder_path).expect(ERROR_MSG_FOLDER_CREATE);
                    }
                    //копируем файл
                    let dest_path = folder_path.join(path.file_name().unwrap());
                    fs::copy(&path, &dest_path).expect(ERROR_MSG_COPY_FILE);
                    fs::remove_file(&path).expect(ERROR_MSG_FILE_DELETE);
                }
            }
        }
    }
}
fn read_directory() -> Box<Path> {
    println!("Укажите директорию для обработки файлов:");
    let mut directory_path = String::new();
    io::stdin()
        .read_line(&mut directory_path)
        .expect(ERROR_MSG_LINE_READ);
    let directory_path = directory_path.trim();

    let directory_path = Path::new(directory_path);
    if !directory_path.is_dir() {
        eprintln!("{}", ERROR_MSG_LINE_DIR);
        std::process::exit(1);
    }
    Box::from(directory_path)
}

fn read_mask() -> Box<HashSet<String>> {
    println!(
        "Введите шаблоны для поиска в имени файла, разделяя их символами '--', если необходимо:"
    );
    let mut extension_mask_input = String::new();
    io::stdin()
        .read_line(&mut extension_mask_input)
        .expect(ERROR_MSG_LINE_READ);

    let extension_masks: HashSet<String> = extension_mask_input
        .split(("--"))
        //.filter(|s| s.is_empty())
        //.map(|s| s.trim().to_lowercase())
        .map(|s| s.trim().to_string())
        .collect();

    Box::from(extension_masks)
}
