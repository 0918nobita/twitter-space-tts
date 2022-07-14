#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", "こんにちは"), ("speaker", "0")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:?}", res);
    Ok(())
}
