use std::{env, fs, io};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::path::PathBuf;
use regex::{Regex, Error as RegexError};

const SAN_REGEX_STRING: &str = r#"[\/:*?"><|]"#;

pub fn get_exe_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let parent_dir = exe_path.parent()
        .ok_or("failed to get path of executable")?;
    let exe_path_buf = PathBuf::from(parent_dir);
    Ok(exe_path_buf)
}

pub fn get_ffmpeg_path() -> Result<PathBuf, Box<dyn Error>> {
    let p = PathBuf::from("./");
    let exe_path = get_exe_path()?;
    let ffmpeg_path = p.join(exe_path).join("ffmpeg");
    Ok(ffmpeg_path)
}

fn contains(lines: &[String], value: &str) -> bool {
    lines.iter().any(|s| s.to_lowercase() == value.to_lowercase())
}

fn read_text_file_lines(filename: &str) -> Result<Vec<String>, IoError> {
    let f = File::open(filename)?;
    let br = BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();
    for result in br.lines() {
        match result {
            Ok(line) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    lines.push(trimmed.to_string());
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(lines)
}

pub fn clean_url(url: &str) -> String {
    let trimmed = url.trim();
    let stripped = trimmed.strip_suffix('/').unwrap_or(&trimmed);
    stripped.to_string()
}

pub fn process_urls(urls: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut processed: Vec<String> = Vec::new();
    let mut text_paths: Vec<String> = Vec::new();

    for url in urls {
        if url.ends_with(".txt") {
            if contains(&text_paths, &url) {
                continue;
            }
            let text_lines = read_text_file_lines(&url)?;
            for text_line in text_lines {
                let cleaned_line = clean_url(&text_line);
                if !contains(&processed, &cleaned_line) {
                    processed.push(cleaned_line);
                }
            }
            text_paths.push(url.clone());
        } else {
            let cleaned_line = clean_url(&url);
            if !contains(&processed, &cleaned_line) {
                processed.push(cleaned_line);
            }
        }
    }

    Ok(processed)
}

pub fn file_exists(file_path: &PathBuf) -> Result<bool, IoError> {
    match fs::metadata(file_path) {
        Ok(meta) => Ok(meta.is_file()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(err)
            }
        }
    }
}

pub fn sanitise(filename: &str, trim_periods: bool) -> Result<String, RegexError> {
    let re = Regex::new(SAN_REGEX_STRING)?;
    let sanitised = re.replace_all(filename, "_");

    let result = if trim_periods {
        sanitised.trim().trim_end_matches('.')
    } else {
        sanitised.trim_start()
    };

    Ok(result.to_string())
}


pub fn format_track_number(track_num: u16, track_total: u16) -> String {
    let padding = track_total.to_string().len();
    format!("{:0width$}", track_num, width = padding)
}

pub fn append_to_path_buf(path: &PathBuf, to_append: &str) -> PathBuf {
    let path_str = path.to_string_lossy();
    let new_path_str = format!("{}{}", path_str, to_append);
    PathBuf::from(new_path_str)
}