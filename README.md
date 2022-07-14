# Twitter スペースでコメントを読み上げるツール

VOICEVOX から得た音声データの再生には portaudio を使用しています。

## 起動手順

- VOICEVOX を起動
- 環境変数の設定
    - `TW_AUTH_TOKEN` で Authorization Token を指定
    - `AUDIO_DEVICE` で出力デバイス名を指定 (例: `Soundflower (2ch)`)
- 以下のコマンドを実行

```bash
cargo run
```
