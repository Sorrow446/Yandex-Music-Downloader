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
#[serde(rename_all = "camelCase")]
pub struct LyricsInfo {
    pub has_available_sync_lyrics: bool,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub artists: Vec<Artist>,
    pub id: String,
    pub title: String,
    pub available: bool,
    pub lyrics_info: LyricsInfo,
    pub version: Option<String>,
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
    pub genre: Option<String>,
    pub labels: Vec<Label>,
    pub version: Option<String>,
    pub volumes: Vec<Vec<Volume>>,
    pub year: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResult {
    pub download_url: String,
}

#[derive(Deserialize)]
pub struct AlbumMeta {
    pub result: AlbumResult,
}

#[derive(Deserialize)]
pub struct LyricsMeta {
    pub result: LyricsResult,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylist {
    pub playlist_uuid: String,
    pub kind: u32,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylistData {
    pub playlist: UserPlaylist,
    // pub track_count: u32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylistItem {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: UserPlaylistData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylistTab {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Vec<UserPlaylistItem>,
}

#[derive(Deserialize)]
pub struct UserPlaylistsMetaResult {
    pub tabs: Vec<UserPlaylistTab>,
}

#[derive(Deserialize)]
pub struct UserPlaylistsMeta {
    pub result: UserPlaylistsMetaResult,
}

#[derive(Deserialize)]
pub struct Owner {
    pub login: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistTrack {
    pub id: String,
    pub title: String,
    pub available: bool,
    pub lyrics_info: LyricsInfo,
    pub albums: Vec<AlbumResultInPlaylist>,
    pub artists: Vec<Artist>,
    pub cover_uri: String,
    pub version: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResultInPlaylist {
    pub title: String,
    pub artists: Vec<Artist>,
    pub available: bool,
    pub genre: Option<String>,
    pub labels: Vec<Label>,
    pub version: Option<String>,
    pub year: u16,
}

#[derive(Deserialize)]
pub struct PlaylistTrackItem {
    pub track: PlaylistTrack,
}

#[derive(Deserialize)]
pub struct PlaylistMetaResult {
    pub available: bool,
    pub owner: Owner,
    pub title: String,
    // pub visibility: String,
    pub tracks: Vec<PlaylistTrackItem>,
}

#[derive(Deserialize)]
pub struct PlaylistMeta {
    pub result: PlaylistMetaResult,
}

#[derive(Deserialize)]
pub struct ArtistMetaAlbum {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct ArtistMetaArtist {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ArtistMetaResult {
    pub albums: Vec<ArtistMetaAlbum>,
    pub artist: ArtistMetaArtist,
}

#[derive(Deserialize)]
pub struct ArtistMeta {
    pub result: ArtistMetaResult,
}