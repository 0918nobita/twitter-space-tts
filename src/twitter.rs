use serde::Deserialize;

pub struct TwitterConfig {
    pub authorization_token: String,
}

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
struct DetailedTweet {
    id: String,
    author_name: String,
    text: String,
}

async fn search(
    query: String,
    since_id: Option<String>,
    tw_config: &TwitterConfig,
) -> Result<Vec<DetailedTweet>, String> {
    let client = reqwest::Client::new();

    let mut query = vec![
        ("query", query),
        ("user.fields", "name".to_owned()),
        ("expansions", "author_id".to_owned()),
    ];

    if let Some(since_id) = since_id {
        query.push(("since_id", since_id));
    }

    let res = client
        .get("https://api.twitter.com/2/tweets/search/recent")
        .query(&query)
        .header(
            "Authorization",
            format!("Bearer {}", tw_config.authorization_token),
        )
        .send()
        .await
        .map_err(|err| format!("Failed to fetch tweets from Twitter API v2\n{}", err))?;

    if !res.status().is_success() {
        eprintln!("{}, skipped", res.status());
        return Ok(vec![]);
    }

    let res = res
        .json::<serde_json::Value>()
        .await
        .map_err(|err| format!("Failed to parse response of Twitter API v2\n{}", err))?;

    let users: Vec<User> =
        serde_json::from_value(res["includes"]["users"].clone()).map_err(|err| {
            format!(
                "Failed to deserialize `includes.users` field of response\n{}",
                err
            )
        })?;

    let tweets: Vec<Tweet> = serde_json::from_value(res["data"].clone())
        .map_err(|err| format!("Failed to deserialize `data` field\n{}", err))?;

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

pub fn watch_latest_tweet(send: tokio::sync::mpsc::Sender<String>, tw_config: TwitterConfig) {
    let re = regex::Regex::new(r"https?://[A-Za-z0-9!\?/\+\-_~=;.,*&@#$%\(\)'\[\]]+").unwrap();

    tokio::spawn(async move {
        let mut latest_tweet_id: Option<String> = None;

        loop {
            if let Ok(tweets) = search(
                "#0918nobitaのスペース".to_owned(),
                latest_tweet_id.clone(),
                &tw_config,
            )
            .await
            {
                if let Some(first) = tweets.first() {
                    latest_tweet_id = Some(first.id.clone());
                }

                for tweet in tweets.iter().rev() {
                    let msg = re.replace_all(&tweet.text, "").to_string();
                    let msg = msg.replace("#0918nobitaのスペース", "");
                    send.send(format!(
                        "{}さんのツイート。{}。ボイスヴォックスで読み上げました。",
                        tweet.author_name, msg
                    ))
                    .await
                    .expect("Failed to send");
                }
            }

            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
}
