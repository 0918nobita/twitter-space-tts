# Twitter スペースでコメントを読み上げるツール

VOICEVOX を起動し、環境変数 `TW_AUTH_TOKEN` をセットしたうえで、以下のコマンドを実行することで動作します。

```bash
cargo run
```

VOICEVOX から得た音声データの再生には portaudio を使用しています。
