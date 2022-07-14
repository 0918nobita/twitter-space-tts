use std::env;
use tokio::sync::mpsc;
use twitter_space_tts::{tts, twitter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tw_auth_token = env::var("TW_AUTH_TOKEN").expect("TW_AUTH_TOKEN is not set");
    let audio_device = env::var("AUDIO_DEVICE").expect("AUDIO_DEVICE is not set");

    let (send, recv) = mpsc::channel::<String>(100);

    twitter::watch_latest_tweet(send, tw_auth_token);

    tts::speak_each_tweet(recv, audio_device).await;

    Ok(())
}
