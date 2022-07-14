use tokio::sync::mpsc;
use twitter_space_tts::{tts, twitter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (send, recv) = mpsc::channel::<String>(100);

    twitter::watch_latest_tweet(send);

    tts::speak_each_tweet(recv).await;

    Ok(())
}
