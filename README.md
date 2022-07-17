# Twitter スペースでコメントを読み上げるツール

[![Lint](https://github.com/0918nobita/twitter-space-tts/actions/workflows/lint.yml/badge.svg)](https://github.com/0918nobita/twitter-space-tts/actions/workflows/lint.yml)

VOICEVOX から得た音声データの再生には PortAudio を使用しています。  
起動した時点の1分前から送信されたツイートを時系列順に読み上げます。

## macOS で起動するための準備

Homebrew 経由で PortAudio をインストールしてください。

```bash
brew install portaudio
```

## Windows で起動するための準備

PortAudio をソースコードからビルドしてください。

- [PortAudio 公式サイト](http://files.portaudio.com/download.html)から `pa_stable_v190700_20210406.tgz` をダウンロードし、解凍する
- [ASIO SDK](https://www.steinberg.net/developers/) をダウンロードし、解凍する
- ↑を PortAudio 側の `portaudio/src/hostapi/asio/ASIOSDK` にコピー・リネームする
- `portaudio/build/msvc/portaudio.sln` を Visual Studio 2022 で開き、ソリューション・プロジェクトのアップグレードを要求されたら「OK」を選択する
- ソリューションを x64, Release でビルドする
- `portaudio/build/msvc/x64/Release/portaudio_x64.dll` をこのプロジェクトのルートにコピーする
- `portaudio/build/msvc/x64/Release/portaudio_x64.lib` をこのプロジェクトの `lib` ディレクトリにコピーし、`portaudio.lib` にリネームする

## 起動手順

- VOICEVOX を起動
- 環境変数 `TW_AUTH_TOKEN` で Twitter API v2 の認可トークンを指定
- 以下のコマンドを実行

```bash
# cargo run -- [検索クエリ]
cargo run -- "#0918nobitaのスペース"
```

`--audio-device` で音声出力デバイスを指定できます。省略した場合デフォルトのデバイスで再生されます。

```bash
cargo run -- "#0918nobitaのスペース" --audio-device "Soundflower (2ch)"
```

対話形式で音声出力デバイスを選択する場合、`--select-audio-device` を指定してください。  
`--audio-device` オプションとの併用はできません。

```bash
cargo run -- "#0918nobitaのスペース" --select-audio-device
```

`--verbose` を指定すると詳細な動作ログを確認できます。

```bash
cargo run -- "#0918nobitaのスペース" --verbose
```
