use crate::api::structs::*;

use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use sha2::Sha256;
use base64::engine::{general_purpose, Engine};
use hmac::{Hmac, Mac};
use hmac::digest::crypto_common::InvalidLength as CryptoInvalidLength;

use reqwest::blocking::{Client, Response as ReqwestResp};
use reqwest::Error as ReqwestErr;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION, RANGE};

const BASE_URL: &str = "https://api.music.yandex.net";
const SECRET: &str = "kzqU4XhfCaY6B6JTHODeq5";
const YANDEX_USER_AGENT: &str = "YandexMusicDesktopAppWindows/5.23.2";

type HmacSha256 = Hmac<Sha256>;

pub struct YandexMusicClient {
    c: Client,
    pub login: String,
    token: String,
}

impl YandexMusicClient {
    pub fn new(token: &str) -> Result<YandexMusicClient, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(YANDEX_USER_AGENT));

        let c = Client::builder()
            .default_headers(headers)
            .build()?;


        let mut yandex_client = YandexMusicClient {
            c,
            token: format!("OAuth {}", token),
            login: String::new(),
        };

        let user_info = yandex_client.get_user_info()?;
        if !user_info.has_plus {
            return Err("active plus subscription required".into());
        }

        yandex_client.login = user_info.login;
        Ok(yandex_client)
    }

    pub fn get_user_info(&mut self) -> Result<UserInfoResult, ReqwestErr> {
        let url = format!("{}/account/about", BASE_URL);
        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: UserInfo = resp.json()?;
        Ok(meta.result)
    }

    pub fn get_album_meta(&mut self, album_id: &str) -> Result<AlbumResult, ReqwestErr> {
        let url = format!("{}/albums/{}/with-tracks", BASE_URL, album_id);
        let resp = self.c.get(url)
            // Auth header not needed, but the Win app does send one.
            .header(AUTHORIZATION, &self.token)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: AlbumMeta = resp.json()?;
        Ok(meta.result)
    }

    fn get_unix_timestamp(&self) -> Result<String, SystemTimeError> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                let timestamp_str = n.as_secs().to_string();
                Ok(timestamp_str)
            },
            Err(e) => Err(e),
        }
    }

    fn create_lyrics_signature(&mut self, ts: &str, track_id: &str) -> Result<String, CryptoInvalidLength> {
        let msg = format!("{}{}", track_id, ts);

        let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes())?;
        mac.update(msg.as_bytes());

        let result = mac.finalize();
        let hmac_bytes = result.into_bytes();
        let base64_encoded = general_purpose::STANDARD.encode(hmac_bytes);

        Ok(base64_encoded)
    }

    pub fn get_lyrics_meta(&mut self, track_id: &str, timed: bool) -> Result<LyricsResult, Box<dyn Error>> {
        let url = format!("{}/tracks/{}/lyrics", BASE_URL, track_id);
        let ts = self.get_unix_timestamp()?;

        let format = if timed { "LRC" } else { "TEXT" };
        let signature = self.create_lyrics_signature(&ts, track_id)?;

        let params: HashMap<&str, &str> = HashMap::from([
            ("timeStamp", ts.as_str()),
            ("trackId", track_id),
            ("format", format),
            ("sign", &signature),
        ]);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .query(&params)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: LyricsMeta = resp.json()?;
        Ok(meta.result)
    }

    // :)
    fn create_signature(&mut self, ts: &str, track_id: &str, quality: &str) -> Result<String, CryptoInvalidLength> {
        let msg = format!("{}{}{}flacaache-aacmp3raw", ts, track_id, quality);

        let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes())?;
        mac.update(msg.as_bytes());

        let result = mac.finalize();
        let hmac_bytes = result.into_bytes();
        let base64_encoded = general_purpose::STANDARD.encode(hmac_bytes);

        Ok(base64_encoded[..base64_encoded.len() - 1].to_string())
    }

    pub fn get_file_info(&mut self, track_id: &str, quality: &str) -> Result<DownloadInfo, Box<dyn Error>> {
        let url = format!("{}/get-file-info", BASE_URL);

        let ts = self.get_unix_timestamp()?;
        let signature = self.create_signature(&ts, track_id, quality)?;

        let params: HashMap<&str, &str> = HashMap::from([
            ("ts", ts.as_str()),
            ("trackId", track_id),
            ("quality", quality),
            ("codecs", "flac,aac,he-aac,mp3"),
            ("transports", "raw"),
            ("sign", &signature),
        ]);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .query(&params)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: FileInfo = resp.json()?;
        Ok(meta.result.download_info)
    }

    pub fn get_user_playlists_meta(&mut self) -> Result<UserPlaylistsMetaResult, Box<dyn Error>> {
        let url = format!(
            "{}/landing-blocks/collection/playlists-liked-and-playlists-created", BASE_URL);

        // Not sure what the max is.
        let params: HashMap<&str, &str> = HashMap::from([
            ("count", "1000"),
        ]);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .query(&params)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: UserPlaylistsMeta = resp.json()?;
        Ok(meta.result)
    }

    pub fn get_other_user_playlist_meta(&mut self, username: &str, id: &str) -> Result<OtherUserPlaylistMetaResult, Box<dyn Error>> {
        let url = format!("{}/users/{}/playlists/{}", BASE_URL, username, id);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: OtherUserPlaylistMeta = resp.json()?;
        Ok(meta.result)
    }

    pub fn get_playlist_meta(&mut self, uuid: &str) -> Result<PlaylistMetaResult, Box<dyn Error>> {
        let url = format!("{}/playlist/{}", BASE_URL, uuid);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .send()?;

        resp.error_for_status_ref()?;

        let meta: PlaylistMeta = resp.json()?;
        Ok(meta.result)
    }

    pub fn get_artist_meta(&mut self, artist_id: &str) -> Result<ArtistMetaResult, Box<dyn Error>> {
        let url = format!("{}/artists/{}", BASE_URL, artist_id);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .send()?;

        resp.error_for_status_ref()?;

        let meta: ArtistMeta = resp.json()?;
        Ok(meta.result)
    }

    pub fn get_user_favourites_meta(&mut self) -> Result<UserFavouritesMeta, Box<dyn Error>> {
        let url = format!("{}/landing/block/likes-and-history", BASE_URL);

        let resp = self.c.get(url)
            .header(AUTHORIZATION, &self.token)
            .header("X-Yandex-Music-Client", YANDEX_USER_AGENT)
            .send()?;

        resp.error_for_status_ref()?;
        let meta: UserFavouritesMeta = resp.json()?;
        Ok(meta)
    }

    pub fn get_file_resp(&mut self, url: &str, with_range: bool) -> Result<ReqwestResp, ReqwestErr> {
        let mut req = self.c.get(url);
        if with_range {
            req = req.header(RANGE, "bytes=0-")
        }
        let resp = req.send()?;
        resp.error_for_status_ref()?;
        Ok(resp)
    }

}