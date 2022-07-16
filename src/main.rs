use clap::Parser;
use serde::Deserialize;
use twitter_space_tts::{launch, tts, twitter};

#[derive(Deserialize)]
struct Config {
    tw_auth_token: String,
}

#[derive(Parser)]
struct Args {
    search_query: String,

    #[clap(long)]
    audio_device: Option<String>,

    #[clap(long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut builder = env_logger::builder();

    if args.verbose {
        builder.filter(Some("twitter_space_tts"), log::LevelFilter::Trace);
    }

    builder.init();

    let config: Config = envy::from_env().unwrap_or_else(|_| {
        log::error!("Failed to load config from environment variables");
        std::process::exit(1);
    });

    let tw_config = twitter::TwitterConfig {
        authorization_token: config.tw_auth_token,
        search_query: args.search_query,
    };

    let tts_config = tts::TTSConfig {
        audio_output_device: args.audio_device,
    };

    launch(tw_config, &tts_config).await;
}
