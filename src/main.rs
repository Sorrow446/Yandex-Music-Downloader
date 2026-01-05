mod structs;

use std::error::Error;
use std::{env, fs, io};
use std::path::PathBuf;
use rusty_leveldb::{Options, DB};
use crate::structs::Token;

const KEY: [u8; 35] = [
    95, 109, 117, 115, 105, 99, 45, 97, 112, 112, 108, 105, 99, 97, 116, 105,
    111, 110, 58, 47, 47, 100, 101, 115, 107, 116, 111, 112, 0, 1, 111, 97,
    117, 116, 104
];

fn copy_dir(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = dst.clone().join(entry.file_name());

        if file_type.is_file() {
            fs::copy(src_path, dst_path)?;
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_db_path() -> Result<PathBuf, Box<dyn Error>> {
    let app_data_path: PathBuf = env::var("appdata")?.into();
    let path = app_data_path.join(r"YandexMusic\Local Storage\leveldb\");
    Ok(path)
}

#[cfg(target_os = "linux")]
fn get_db_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_path: PathBuf = env::var("HOME")?.into();
    let path = home_path.join(".config").join("yandex-music/Local Storage/leveldb/");
    Ok(path)
}

#[cfg(target_os = "macos")]
fn get_db_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_path: PathBuf = env::var("HOME")?.into();
    let path = home_path.join("Library/Application Support/YandexMusic/Local Storage/leveldb/");
    Ok(path)
}

fn read_token(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let opts = Options::default();
    let mut db = DB::open(&path, opts)?;


    let token = match db.get(&KEY) {
        Some(v) => {
            let obj: Token = serde_json::from_slice(&v[1..])?;
            obj.value
        }
        None => Err("key missing in db")?,
    };

    fs::remove_dir_all(path)?;
    Ok(token)
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = get_db_path()?;
    let copy_path = db_path.join(r"tmp");

    copy_dir(&db_path, &copy_path)?;
    let token = read_token(&copy_path)?;
    println!("{}", token);

    println!("Press enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
