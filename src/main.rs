use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let synthesis_query = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", "こんにちは"), ("speaker", "0")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let wav_bytes = client
        .post("http://localhost:50021/synthesis")
        .query(&[("speaker", "0")])
        .json(&synthesis_query)
        .send()
        .await?
        .bytes()
        .await?;

    std::fs::File::create("output.wav")?.write_all(&wav_bytes)?;

    Ok(())
}
