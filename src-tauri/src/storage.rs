use std::fs;
use std::path::Path;
use tauri::api::path::local_data_dir;

const DirName:std::str = "Files";

#[derive(serde::Serialize, Debug)]
pub struct StorageData {
    pub data: serde_json::Value,
    pub status: bool,
}

#[tauri::command]
pub fn write_data(name: &String, data: &Vec<u8>) -> Result<(), String> {
    let local_dir = match local_data_dir() {
        Some(dir) => dir,
        None => return Err("get local dir error".to_string()),
    };

    let data_dir = Path::new(&local_dir).join("Files");

    if let Err(err) = fs::create_dir_all(&data_dir) {
        return Err(format!("create dir error::{}", err.to_string()));
    }

    if let Err(err) = fs::write(data_dir.join(name), data) {
        return Err(format!("write file error::{}", err.to_string()));
    }

    Ok(())
}

#[tauri::command]
pub fn read_data(name: &String) -> Result<Vec<u8>, String> {
    let local_dir = match local_data_dir() {
        Some(dir) => dir,
        None => return Err("get local dir error".to_string()),
    };

    let data_dir = Path::new(&local_dir).join("Files");

    match fs::read(data_dir.join(name)) {
        Ok(data) => return Ok(data),
        Err(err) => Err(format!("write file error::{}", err.to_string())),
    }
}

#[tauri::command]
pub fn delete_storage_data(key: String) {
    let storage_dir = Path::new(&local_data_dir().unwrap()).join("Xplorer");

    if let Ok(_) = fs::remove_file(storage_dir.join(key)) {}
}
