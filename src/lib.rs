pub mod tts;
pub mod twitter;

pub async fn launch(tw_config: twitter::TwitterConfig, tts_config: &tts::TTSConfig) {
    let (send, recv) = tokio::sync::mpsc::channel::<String>(100);

    twitter::watch_latest_tweet(send, tw_config);

    tts::speak_each_tweet(recv, tts_config).await;
}
