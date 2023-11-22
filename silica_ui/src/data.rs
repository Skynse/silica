use std::path::{Path, PathBuf};

pub fn get_data_dir() -> PathBuf {
    // if windows, use %APPDATA% roaming then silica
    // if linux, use $XDG_DATA_HOME/silica

    let mut data_dir = PathBuf::new();
    if cfg!(windows) {
        data_dir.push(std::env::var("APPDATA").unwrap());
    } else {
        data_dir.push(std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            let mut home = PathBuf::new();
            home.push(std::env::var("HOME").unwrap());
            home.push(".local/share");
            home.to_str().unwrap().to_string()
        }));
    }

    data_dir.push("silica");

    data_dir
}

pub fn create_data_dir() {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir).unwrap();
    }

    // create saves dir
    create_save_dir();
}

pub fn create_save_dir() {
    let save_dir = get_save_dir();
    if !save_dir.exists() {
        std::fs::create_dir_all(save_dir).unwrap();
    }
}

pub fn get_save_dir() -> PathBuf {
    let mut save_dir = get_data_dir();
    save_dir.push("saves");
    save_dir
}
