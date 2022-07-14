use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    id: String,
    name: String,
    // username: String,
}

#[derive(Debug, Deserialize)]
struct Tweet {
    author_id: String,
    id: String,
    text: String,
}

#[derive(Debug)]
pub struct DetailedTweet {
    pub id: String,
    pub author_name: String,
    pub text: String,
}

pub async fn search(query: &str) -> Result<Vec<DetailedTweet>, Box<dyn std::error::Error>> {
    let tw_auth_token = std::env::var("TW_AUTH_TOKEN").expect("TW_AUTH_TOKEN is not set");

    let client = reqwest::Client::new();

    let res = client
        .get("https://api.twitter.com/2/tweets/search/recent")
        .query(&[
            ("query", query),
            ("user.fields", "name"),
            ("expansions", "author_id"),
        ])
        .header("Authorization", format!("Bearer {}", tw_auth_token))
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Status: {}", res.status());
        return Ok(Vec::new());
    }

    let res = res.json::<serde_json::Value>().await?;

    let users: Vec<User> = serde_json::from_value(res["includes"]["users"].clone())?;

    let tweets: Vec<Tweet> = serde_json::from_value(res["data"].clone())?;

    let detailed_tweets: Vec<DetailedTweet> = tweets
        .iter()
        .filter_map(|tweet| {
            let user = users.iter().find(|user| user.id == tweet.author_id)?;
            Some(DetailedTweet {
                id: tweet.id.clone(),
                author_name: user.name.clone(),
                text: tweet.text.clone(),
            })
        })
        .collect();

    Ok(detailed_tweets)
}
