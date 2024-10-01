use clap::Parser;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "Yandex Music Downloader")]
pub struct Args {
    #[clap(short, long, help="1 = AAC 64, 2 = AAC 192, 3 = AAC 256 / MP3 320, 4 = FLAC.")]
    pub format: Option<u8>,

    #[clap(short, long, help="Get original covers for tracks; may be large sometimes. true = orignal, false = 1000x1000.")]
    pub get_original_covers: bool,

    #[clap(short, long, help="Keep covers in album folder.")]
    pub keep_covers: bool,

    #[clap(short, long, help="Output path.")]
    pub out_path: Option<PathBuf>,

    #[clap(short, long, help="Sleep between each track processing to prevent potential rate-limiting.")]
    pub sleep: bool,

    #[clap(long, help="Write covers to tracks.")]
    pub write_covers: bool,

    #[clap(long, help="Write timed lyrics when available.")]
    pub write_lyrics: bool,

    #[clap(short, long, num_args = 1.., required = true)]
    pub urls: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    // #[serde(skip_deserializing)]
    pub format: u8,
    #[serde(skip_deserializing)]
    pub format_str: String,
    pub keep_covers: bool,
    pub out_path: PathBuf,
    pub get_original_covers: bool,
    pub token: String,
    pub sleep: bool,
    #[serde(skip_deserializing)]
    pub urls: Vec<String>,
    pub write_covers: bool,
    pub write_lyrics: bool,
}

pub struct ParsedAlbumMeta {
    pub album_title: String,
    pub album_artist: String,
    pub artist: String,
    pub cover_data: Vec<u8>,
    pub genre: String,
    pub has_lyrics: bool,
    pub label: String,
    pub title: String,
    pub track_num: u16,
    pub track_total: u16,
    pub year: u16,
}