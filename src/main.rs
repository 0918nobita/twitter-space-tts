use clap::{ArgGroup, Parser};
use log::{error, trace};
use twitter_space_tts::{launch, tts, twitter};

#[derive(Parser)]
#[clap(group(
    ArgGroup::new("audio-device-group")
        .required(false)
        .multiple(false)
        .args(&["audio-device", "select-audio-device"])
))]
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
    /*
    let selected = dialoguer::Select::new()
        .with_prompt("Select audio output device")
        .default(0)
        .items(&["Foo", "Bar", "Baz"])
        .interact()
        .unwrap();
    println!("You selected: {}", selected);
    */

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
    let tw_auth_token = std::env::var("TW_AUTH_TOKEN").unwrap_or_else(|_| {
        error!("TW_AUTH_TOKEN environment variable is not set");
        std::process::exit(1);
    });

    let tw_config = twitter::TwitterConfig {
        authorization_token: tw_auth_token,
        search_query: args.search_query,
    };

    let tts_config = tts::TTSConfig {
        audio_output_device: args.audio_device,
    };

    launch(tw_config, &tts_config).await;
}
