use std::env;
use twitter_space_tts::{launch, tts, twitter};

#[tokio::main]
async fn main() {
    let tw_auth_token = env::var("TW_AUTH_TOKEN").expect("TW_AUTH_TOKEN is not set");

    let audio_device = env::var("AUDIO_DEVICE").expect("AUDIO_DEVICE is not set");

    let tw_config = twitter::TwitterConfig {
        authorization_token: tw_auth_token,
    };

    let tts_config = tts::TTSConfig {
        audio_output_device: audio_device,
    };

    launch(tw_config, &tts_config).await;
}
