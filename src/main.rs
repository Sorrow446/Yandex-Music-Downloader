use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use regex::Regex;
use std::{thread, time};

use clap::Parser;
use reqwest::Error as ReqwestErr;
use indicatif::{ProgressBar, ProgressStyle};
use metaflac::{Tag as FlacTag, Error as FlacError};
use metaflac::block::PictureType::CoverFront as FLACCoverFront;
use id3::{Error as ID3Error, Tag as Mp3Tag, TagLike, Version};
use id3::frame::{Picture as Mp3Image};
use id3::frame::PictureType::CoverFront as MP3CoverFront;
use mp4ameta::{Tag as Mp4Tag, Data as Mp4Data, Fourcc, Error as MP4Error};

use crate::api::client::YandexMusicClient;
use crate::api::structs::{AlbumResult, Artist, Label, Volume};
use crate::structs::{Args, Config, ParsedAlbumMeta};

mod api;
mod structs;
mod utils;

const REGEX_STR: &str = r#"^https://music.yandex.ru/album/(\d+)$"#;
const BUF_SIZE: usize = 1024 * 1024;

#[cfg(target_os = "windows")]
const IS_WINDOWS: bool = true;

#[cfg(not(target_os = "windows"))]
const IS_WINDOWS: bool = false;

fn read_config() -> Result<Config, Box<dyn Error>> {
    let exe_path = utils::get_exe_path()?;
    let config_path = exe_path.join("config.toml");
    let data = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&data)?;
    Ok(config)
}

fn resolve_format(fmt: u8) -> Option<String> {
    match fmt {
        1 => Some("lq".to_string()),
        2 => Some("nq".to_string()),
        3 => Some("hq".to_string()),
        4 => Some("lossless".to_string()),
        _ => None,
    }
}

fn parse_config() -> Result<Config, Box<dyn Error>> {
    let mut config = read_config()?;
    if config.token.trim().is_empty() {
        return Err("token can't be empty".into())
    }

    let args = Args::parse();
    let proc_urls = utils::process_urls(&args.urls)?;

    if args.keep_covers {
        config.keep_covers = args.keep_covers;
    }

    if args.write_covers {
        config.write_covers = args.write_covers;
    }

    config.format = args.format.unwrap_or(config.format);
    config.out_path = args.out_path.unwrap_or(config.out_path);

    config.out_path.push("Yandex Music downloads");

    config.format_str = resolve_format(config.format)
        .ok_or("format must be between 1 and 4")?;

    config.urls = proc_urls;
    Ok(config)
}

fn check_url(url: &str, regex: &Regex) -> Option<String> {
    if let Some(capture) = regex.captures(url) {
        if let Some(m) = capture.get(1) {
            let album_id = m.as_str().to_string();
            return Some(album_id);
        }
    }
    None
}
fn parse_artists(artists: &[Artist]) -> String {
    artists.iter()
        .map(|a| a.name.clone())
        .collect::<Vec<String>>()
        .join(", ")
}

fn parse_labels(labels: &[Label]) -> String {
    labels.iter()
        .map(|l| l.name.clone())
        .collect::<Vec<String>>()
        .join(", ")
}

fn parse_album_meta(meta: &AlbumResult, track_total: u16) -> ParsedAlbumMeta {
    ParsedAlbumMeta {
        album_artist: parse_artists(&meta.artists),
        album_title: meta.title.clone(),
        artist: String::new(),
        cover_data: Vec::new(),
        genre: meta.genre.clone(),
        title: String::new(),
        track_num: 0,
        track_total,
        label: parse_labels(&meta.labels),
        year: meta.year,
    }
}

fn parse_track_meta(meta: &mut ParsedAlbumMeta, track_meta: &Volume, track_num: u16) {
    meta.artist =  parse_artists(&track_meta.artists);
    meta.title = track_meta.title.clone();
    meta.track_num = track_num;
}

fn get_cover_data(c: &mut YandexMusicClient, url: &str) -> Result<Vec<u8>, Box<ReqwestErr>> {
    let replaced_url = url.replace("/%%", "/1000x1000");
    let full_url = format!("https://{}", replaced_url);

    let resp = c.get_file_resp(&full_url, false)?;
    let body_bytes = resp.bytes()?;
    let body_vec: Vec<u8> = body_bytes.into_iter().collect();
    Ok(body_vec)
}

fn write_cover(cover_data: &[u8], album_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let cover_path = album_path.join("folder.jpg");
    let mut f = File::create(cover_path)?;
    f.write_all(cover_data)?;
    Ok(())
}

fn parse_specs(codec: &str, bitrate: u16) -> Option<(String, String)> {
    match codec {
        "flac" => Some((
            "FLAC".to_string(),
            ".flac".to_string()
        )),
        "mp3" => Some((
            format!("{} Kbps MP3", bitrate),
            ".mp3".to_string()
        )),
        "aac" | "he-aac" => Some((
            format!("{} Kbps AAC", bitrate),
            ".m4a".to_string()
        )),
        _ => None,
    }
}

fn download_track(c: &mut YandexMusicClient, url: &str, out_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut resp = c.get_file_resp(url, true)?;

    let total_size = resp
        .content_length()
        .ok_or("no content length header")?;

    let f = File::create(out_path)?;
    let mut writer = BufWriter::new(f);
    let mut buf = vec![0u8; BUF_SIZE];

    let mut downloaded: usize = 0;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% at {binary_bytes_per_sec}, {bytes}/{total_bytes} (ETA: {eta})")?
        .progress_chars("#>-"));

    loop {
        let n = &resp.read(&mut buf)?;
        if n.to_owned() == 0 {
            break;
        }
        writer.write_all(&buf[..n.to_owned()])?;
        downloaded += n;
        pb.set_position(downloaded as u64);
    }

    pb.finish();
    Ok(())
}

fn set_vorbis(tag: &mut metaflac::Tag, key: &str, value: &str) {
    if !value.is_empty() {
        tag.set_vorbis(key, vec!(value));
    }
}

fn set_vorbis_num(tag: &mut metaflac::Tag, key: &str, n: u16) {
    if n > 0 {
        tag.set_vorbis(key, vec!(n.to_string()));
    }
}

fn write_flac_tags(track_path: &PathBuf, meta: &ParsedAlbumMeta) -> Result<(), FlacError> {
    let mut tag = FlacTag::read_from_path(&track_path)?;

    set_vorbis(&mut tag, "ALBUM", &meta.album_title);
    set_vorbis(&mut tag, "ALBUMARTIST", &meta.album_artist);
    set_vorbis(&mut tag, "ARTIST", &meta.artist);
    set_vorbis(&mut tag, "GENRE", &meta.genre);
    set_vorbis(&mut tag, "LABEL", &meta.label);
    set_vorbis(&mut tag, "TITLE", &meta.title);

    set_vorbis_num(&mut tag, "TRACKNUMBER", meta.track_num);
    set_vorbis_num(&mut tag, "TRACKTOTAL", meta.track_total);
    set_vorbis_num(&mut tag, "YEAR", meta.year);

    if !meta.cover_data.is_empty() {
        tag.add_picture("image/jpeg", FLACCoverFront, meta.cover_data.clone());
    }

    tag.save()?;
    Ok(())
}

fn write_mp3_tags(track_path: &PathBuf, meta: &ParsedAlbumMeta) -> Result<(), ID3Error> {
    let mut tag = Mp3Tag::new();

    tag.set_album(&meta.album_title);
    tag.set_album_artist(&meta.album_artist);
    tag.set_artist(&meta.artist);
    tag.set_genre(&meta.genre);
    tag.set_title(&meta.title);
    tag.set_track(meta.track_num as u32);
    tag.set_total_tracks(meta.track_total as u32);
    if meta.year > 0 {
        tag.set_year(meta.year as i32);
    }

    if !meta.cover_data.is_empty() {
        let pic = Mp3Image {
            mime_type: "image/jpeg".to_string(),
            picture_type: MP3CoverFront,
            description: String::new(),
            data: meta.cover_data.clone(),
        };
        tag.add_frame(pic);
    }

    tag.write_to_path(track_path, Version::Id3v24)?;
    Ok(())
}

fn write_mp4_tags(track_path: &PathBuf, meta: &ParsedAlbumMeta) -> Result<(), MP4Error> {
    let mut tag = Mp4Tag::read_from_path(&track_path)?;

    tag.set_album(&meta.album_title);
    tag.set_album_artist(&meta.album_artist);
    tag.set_artist(&meta.artist);
    tag.set_genre(&meta.genre);
    tag.set_title(&meta.title);
    tag.set_track(meta.track_num, meta.track_total);
    if meta.year > 0 {
        tag.set_year(meta.year.to_string());
    }

    let covr = Fourcc(*b"covr");
    if !meta.cover_data.is_empty() {
        tag.add_data(covr, Mp4Data::Jpeg(meta.cover_data.clone()));
    }

    tag.write_to_path(&track_path)?;
    Ok(())
}

fn write_tags(track_path: &PathBuf, codec: &str, meta: &ParsedAlbumMeta) -> Result<(), Box<dyn Error>> {
    match codec {
        "flac" => write_flac_tags(track_path, meta)?,
        "mp3" => write_mp3_tags(track_path, meta)?,
        "aac" | "he-aac" => write_mp4_tags(track_path, meta)?,
        _ => {},
    }
    Ok(())
}

fn process_track(c: &mut YandexMusicClient, track_id: &str, meta: &ParsedAlbumMeta, config: &Config, album_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let info = c.get_file_info(track_id, &config.format_str)?;
    let (specs, file_ext) = parse_specs(&info.codec, info.bitrate)
        .ok_or_else(|| format!("the api returned an unknown codec: {}", info.codec))?;

    println!("Track {} of {}: {} - {}", meta.track_num, meta.track_total, meta.title, specs);
    // let (specs, ext) = if let Some((specs, ext)) = parse_specs(&info.codec, info.bitrate) {
    //     (specs, ext)
    // } else {
    //     let err_str = format!("the api returned an unknown codec: {}", info.codec);
    //     return Err(err_str.into());
    // };

    let padding = utils::format_track_number(meta.track_num, meta.track_total);
    let san_track_fname = format!(
        "{}. {}", padding, utils::sanitise(&meta.title)?
    );

    let mut track_path_no_ext = album_path.join(san_track_fname);
    let mut track_path = utils::append_to_path_buf(&track_path_no_ext, &file_ext);


    if IS_WINDOWS && track_path.to_string_lossy().len() > 255 {
        track_path_no_ext = album_path.join(padding);
        track_path = utils::append_to_path_buf(&track_path_no_ext, &file_ext);
        println!("Track exceeds max path length; will be renamed like <track_num>.<ext> instead.");
    }

    if utils::file_exists(&track_path)? {
        println!("Track already exists locally.");
        return Ok(());
    }

    let track_path_incomp = utils::append_to_path_buf(&track_path_no_ext, ".incomplete");
    download_track(c, &info.url, &track_path_incomp)?;
    fs::rename(&track_path_incomp, &track_path)?;
    write_tags(&track_path, &info.codec, &meta)?;

    Ok(())

}

fn process_album(c: &mut YandexMusicClient, config: &Config, album_id: &str) -> Result<(), Box<dyn Error>> {
    let meta = c.get_album_meta(album_id)?;
    if !meta.available {
        return Err("album is unavailable".into());
    }

    let track_total = meta.volumes[0].len();
    let mut parsed_meta = parse_album_meta(&meta, track_total as u16);

    let album_folder = format!("{} - {}", parsed_meta.album_artist, parsed_meta.album_title);
    println!("{}", album_folder);

    let san_album_folder = utils::sanitise(&album_folder)?;
    let album_path = config.out_path.join(san_album_folder);
    fs::create_dir_all(&album_path)?;

    let cover_data = get_cover_data(c, &meta.cover_uri)?;

    if config.keep_covers {
        write_cover(&cover_data, &album_path)?;
    }

    if config.write_covers {
        parsed_meta.cover_data = cover_data.clone();
    }

    for (mut track_num, track) in meta.volumes[0].iter().enumerate() {
        track_num += 1;
        if !track.available {
            println!("Track is unavailable.");
            continue;
        }
        parse_track_meta(&mut parsed_meta, track, track_num as u16);
        if let Err(e) = process_track(c, &track.id, &parsed_meta, &config, &album_path) {
            println!("Track failed.\n{:?}", e);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config()
        .expect("failed to parse args/config");
    fs::create_dir_all(&config.out_path)?;

    let regex_comp = Regex::new(REGEX_STR)?;

    let mut c = YandexMusicClient::new(&config.token)?;
    println!("Signed in successfully.\n");

    let url_total = config.urls.len();

    for (mut url_num, url) in config.urls.iter().enumerate() {
        url_num += 1;
        println!("URL {} of {}:", url_num, url_total);

        let album_id = if let Some(album_id) = check_url(url, &regex_comp) {
            album_id
        } else {
            println!("Invalid URL: {}", url);
            continue;
        };

        if let Err(e) = process_album(&mut c, &config, &album_id) {
            println!("Album failed.\n{:?}", e);
        }

        if config.sleep {
            println!("Sleeping...");
            thread::sleep(time::Duration::from_secs(2));
        }

    }

    Ok(())
}







