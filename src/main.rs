#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tweets = twitter_space_tts::twitter::search("0918nobita").await?;
    println!("{:?}", tweets);
    twitter_space_tts::tts::speak("これはコメント読み上げのテストです").await?;
    Ok(())
}
