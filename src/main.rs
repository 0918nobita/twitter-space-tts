use serde::Deserialize;
use twitter_space_tts::{launch, tts, twitter};

#[derive(Deserialize)]
struct Config {
    tw_auth_token: String,
    audio_device: String,
}

#[tokio::main]
async fn main() {
    let config: Config = envy::from_env().expect("Unable to read config");

    let tw_config = twitter::TwitterConfig {
        authorization_token: config.tw_auth_token,
    };

    let tts_config = tts::TTSConfig {
        audio_output_device: config.audio_device,
    };

    launch(tw_config, &tts_config).await;
}
