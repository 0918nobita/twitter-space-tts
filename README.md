# Twitter スペースでコメントを読み上げるツール

VOICEVOX から得た音声データの再生には portaudio を使用しています。  
起動した時点の1分前から送信されたツイートを時系列順に読み上げます。

## 起動手順

- VOICEVOX を起動
- 環境変数の設定
    - `TW_AUTH_TOKEN` で Authorization Token を指定
    - `AUDIO_DEVICE` で出力デバイス名を指定 (例: `Soundflower (2ch)`)
- 以下のコマンドを実行

```bash
# cargo run -- [検索クエリ]
cargo run -- "#0918nobitaのスペース"
```
