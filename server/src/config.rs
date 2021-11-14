use std::env;

pub fn get_host() -> String {
    env::var("HOST").unwrap_or("localhost".to_string())
}

pub fn get_port() -> String {
    env::var("PORT").unwrap_or("8000".to_string())
}

pub fn get_file_storage_path() -> String {
    env::var("FILE_STORAGE_PATH").unwrap_or("../".to_string())
}
