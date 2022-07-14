use tokio::sync::mpsc;
use twitter_space_tts::{tts, twitter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (send, mut recv) = mpsc::channel::<String>(100);

    tokio::spawn(async move {
        let mut latest_tweet_id: Option<String> = None;
        loop {
            if let Ok(tweets) =
                twitter::search("#0918nobitaのスペース".to_owned(), latest_tweet_id.clone()).await
            {
                if let Some(first) = tweets.first() {
                    latest_tweet_id = Some(first.id.clone());
                }
                for tweet in tweets.iter().rev() {
                    send.send(format!(
                        "{}さんのツイート。{}",
                        tweet.author_name,
                        tweet.text.replace("#0918nobitaのスペース", "")
                    ))
                    .await
                    .expect("Failed to send");
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        if let Ok(msg) = recv.try_recv() {
            tts::speak(&msg).await?;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
