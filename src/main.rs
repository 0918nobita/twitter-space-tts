#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    twitter_space_tts::tts::speak("これはコメント読み上げのテストです").await?;
    Ok(())
}
