# Twitter スペースでコメントを読み上げるツール

[![Lint](https://github.com/0918nobita/twitter-space-tts/actions/workflows/lint.yml/badge.svg)](https://github.com/0918nobita/twitter-space-tts/actions/workflows/lint.yml)

VOICEVOX から得た音声データの再生には portaudio を使用しています。  
起動した時点の1分前から送信されたツイートを時系列順に読み上げます。

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

`--verbose` を指定すると詳細な動作ログを確認できます。

```bash
cargo run -- "#0918nobitaのスペース" --verbose
```
