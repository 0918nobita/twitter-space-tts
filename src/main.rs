use clap::{ArgGroup, Parser};
use log::{error, trace};
use std::{env, process};
use twitter_space_tts::{launch, twitter, TTSConfig, TTSOutputDevice};

#[derive(Parser)]
#[clap(
    author,
    version,
    group(
        ArgGroup::new("audio-device-group")
            .required(false)
            .multiple(false)
            .args(&["audio-device", "select-audio-device"])
    )
)]
struct Args {
    search_query: String,

    #[clap(long, short = 'a')]
    audio_device: Option<String>,

    #[clap(long, short = 's')]
    select_audio_device: bool,

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

    trace!("Get config from command line arguments");
    trace!("Search query: {}", args.search_query);
    trace!(
        "Audio device: {}",
        args.audio_device.as_deref().unwrap_or("Not specified")
    );

    trace!("Get Twitter API v2 authorization token from TW_AUTH_TOKEN environment variable");
    let tw_auth_token = env::var("TW_AUTH_TOKEN").unwrap_or_else(|_| {
        error!("TW_AUTH_TOKEN environment variable is not set");
        process::exit(1);
    });

    let tw_config = twitter::TwitterConfig {
        authorization_token: tw_auth_token,
        search_query: args.search_query,
    };

    let tts_config = if args.select_audio_device {
        if args.audio_device.is_some() {
            unreachable!();
        }
        TTSConfig {
            audio_output_device: TTSOutputDevice::SelectInteractively,
        }
    } else if let Some(device_name) = args.audio_device {
        TTSConfig {
            audio_output_device: TTSOutputDevice::Specified(device_name),
        }
    } else {
        TTSConfig {
            audio_output_device: TTSOutputDevice::NotSpecified,
        }
    };

    if let Err(err) = launch(tw_config, &tts_config).await {
        error!("{}", err);
        process::exit(1);
    }
}
