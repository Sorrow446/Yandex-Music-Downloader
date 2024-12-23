# Yandex-Music-Downloader
Yandex Music (Яндекс Музыка) downloader written in Rust with lossless support.
![](https://i.imgur.com/mQrzTfQ.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/Yandex-Music-Downloader/releases)

## Setup
Input token into config file (config.toml).
Configure any other options if needed.
|Option|Info|
| --- | --- |
|token|Required to auth.|
|format|Track download quality. 1 = AAC 64, 2 = AAC 192, 3 = AAC 256 / MP3 320, 4 = FLAC.|
|out_path|Where to download to. Path will be made if it doesn't already exist.|
|keep_covers|Keep covers in album folder.|
|write_covers|Write covers to tracks.|
|sleep|Sleep between each track processing to prevent potential rate-limiting.|
|original_covers|Get original covers for tracks; may be large sometimes. true = orignal, false = 1000x1000|
|track_template|Track filename naming template. Vars: artist, title, track_num, track_num_pad

## Token Acquisition
**Plus subscription required.**    

|Type|Lossless|Lifetime|How to get|
| --- | --- | --- | --- |
|Desktop|yes|1 year|https://github.com/Sorrow446/Yandex-Music-Downloader/tree/token_extractor|
|Android|yes|1 year|Sniff the Android app; look for the Authorization header. `OAuth xxxx...`|
|Web|no|1 month?|https://yandex-music.readthedocs.io/en/main/token.html|


## Supported Media
Wrap any URLs that contain params in double quotes if running on Windows.

|Type|URL example|
| --- | --- |
|Album|`https://music.yandex.ru/album/33134482`
|Artist albums|`https://music.yandex.ru/artist/9838127`, `https://music.yandex.ru/artist/9838127/albums`
|Track|`https://music.yandex.ru/album/2955514/track/25128596`
|User playlist|`https://music.yandex.ru/users/user@gmail.com/playlists/1000`
|User favourites|`https://music.yandex.ru/users/user@gmail.com/playlists/3`

Other users' favourites and playlists are also supported, but they must be set to public.

## Usage
Args take priority over the config file.

Download two albums:   
`ym-dl.exe -u https://music.yandex.ru/album/33134482 https://music.yandex.ru/album/33199228`

Download a single album and from a text file containing links:   
`ym-dl.exe -u https://music.yandex.ru/album/33134482 G:\1.txt`

```
Usage: ym-dl.exe [OPTIONS] --urls <URLS>...

Options:
  -f, --format <FORMAT>      1 = AAC 64, 2 = AAC 192, 3 = AAC 256 / MP3 320, 4 = FLAC.
  -g, --get-original-covers  Get original covers for tracks; may be large sometimes. true = orignal, false = 1000x1000.
  -k, --keep-covers          Keep covers in album folder.
  -o, --out-path <OUT_PATH>  Output path.
  -s, --sleep                Sleep between each track processing to prevent potential rate-limiting.
      --write-covers         Write covers to tracks.
      --write-lyrics         Write timed lyrics when available.
  -u, --urls <URLS>...
  -h, --help                 Print help
```

## Для русских:
Не стесняйтесь открывать вопрос, если у вас есть проблемы с инструментом или вы хотите, чтобы функции были реализованы. Я не говорю по-русски, но могу воспользоваться услугами переводчика.

## Disclaimer
- I will not be responsible for how you use Yandex Music Downloader.    
- Yandex brand and name is the registered trademark of its respective owner.    
- Yandex Music Downloader has no partnership, sponsorship or endorsement with Yandex.
