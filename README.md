# Yandex-Music-Downloader
Yandex Music (Яндекс Музыка) downloader written in Rust with lossless support.
![](https://i.imgur.com/mQrzTfQ.png)

## Setup
Input token into config file.
Configure any other options if needed.
|Option|Info|
| --- | --- |
|token|Required to auth.
|format|Track download quality. 1 = AAC 64, 2 = AAC 192, 3 = AAC 256 / MP3 320, 4 = FLAC.
|out_path|Where to download to. Path will be made if it doesn't already exist.
|keep_covers|Keep covers in album folder.
|write_covers|Write covers to tracks.

## Token Acquisition
The web token won't work. It has to be from one of the following:

From the Windows Yandex Music app:    
- Run the included `extract_token.exe` binary.
The app must be installed and logged in, doesn't have to be running.

From Android:    
- Sniff the Yandex Music app on your Android device or emulator. Look for the Authorization header.
`OAuth xxxx...`

They last for about a year.

## Supported Media
|Type|URL example|
| --- | --- |
|Album|`https://music.yandex.ru/album/33134482`

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
  -o, --out-path <OUT_PATH>  Output path.
  -s, --sleep                Sleep between each track processing to prevent potential rate-limiting.
  -k, --keep-covers          Keep covers in album folder.
  -w, --write-covers         Write covers to tracks.
  -u, --urls <URLS>...
  -h, --help                 Print help
```

## Для русских:
Не стесняйтесь открывать вопрос, если у вас есть проблемы с инструментом или вы хотите, чтобы функции были реализованы. Я не говорю по-русски, но могу воспользоваться услугами переводчика.

## Disclaimer
- I will not be responsible for how you use Yandex Music Downloader.    
- Yandex brand and name is the registered trademark of its respective owner.    
- Yandex Music Downloader has no partnership, sponsorship or endorsement with Yandex.
