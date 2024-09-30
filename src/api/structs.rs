use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResult {
    pub has_plus: bool,
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub result: UserInfoResult,
}

#[derive(Deserialize)]
pub struct Volume {
    pub artists: Vec<Artist>,
    pub id: String,
    pub title: String,
    pub available: bool,
}

#[derive(Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Label {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResult {
    pub title: String,
    pub artists: Vec<Artist>,
    pub available: bool,
    pub cover_uri: String,
    pub genre: String,
    pub labels: Vec<Label>,
    pub volumes: Vec<Vec<Volume>>,
    pub year: u16,
}

#[derive(Deserialize)]
pub struct AlbumMeta {
    pub result: AlbumResult,
}

#[derive(Deserialize)]
pub struct DownloadInfo {
    // pub quality: String,
    pub url: String,
    pub bitrate: u16,
    pub codec: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfoResult {
    pub download_info: DownloadInfo,
}

#[derive(Deserialize)]
pub struct FileInfo {
    pub result: FileInfoResult,
}