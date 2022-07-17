use anyhow::Context;
use log::trace;

pub mod tts;
pub mod twitter;

pub enum TTSOutputDevice {
    NotSpecified,
    Specified(String),
    SelectInteractively,
}

pub struct TTSConfig {
    pub audio_output_device: TTSOutputDevice,
}

pub async fn launch(
    tw_config: twitter::TwitterConfig,
    tts_config: &TTSConfig,
) -> anyhow::Result<()> {
    let (send, recv) = tokio::sync::mpsc::channel::<String>(100);

    twitter::watch_latest_tweet(send, tw_config);

    trace!("Initializing PortAudio");
    let pa = portaudio::PortAudio::new().context("Failed to initialize PortAudio")?;

    let (device_index, device_info) = match &tts_config.audio_output_device {
        TTSOutputDevice::NotSpecified => {
            let device_index = pa.default_output_device()?;
            let device_info = pa.device_info(device_index)?;
            (device_index, device_info)
        }

        TTSOutputDevice::Specified(device_name) => pa
            .devices()
            .context("Failed to enumerate audio devices")?
            .filter_map(|device| device.ok())
            .find(|(_, device_info)| device_info.name == device_name)
            .with_context(|| format!("`{}` device not found", device_name))?,

        TTSOutputDevice::SelectInteractively => {
            let device_index = pa.default_output_device()?;
            let device_names = pa
                .devices()?
                .filter_map(|device| device.ok().map(|(_, device_info)| device_info.name))
                .collect::<Vec<_>>();
            let selected =
                dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Select audio output device")
                    .items(&device_names)
                    .default(device_index.0 as usize)
                    .interact()
                    .unwrap();
            let device_index = portaudio::DeviceIndex(selected as u32);
            let device_info = pa.device_info(device_index)?;
            (device_index, device_info)
        }
    };

    trace!("Selected audio output device: {}", device_info.name);

    let output_params = portaudio::StreamParameters::<f32>::new(
        device_index,
        tts::CHANNELS,
        true,
        device_info.default_low_input_latency,
    );

    pa.is_output_format_supported(output_params, tts::SAMPLE_RATE)?;

    let output_settings =
        portaudio::OutputStreamSettings::new(output_params, tts::SAMPLE_RATE, tts::FRAMES);

    let tts_context = tts::TTSContext {
        pa,
        output_settings,
    };

    tts::speak_each_tweet(recv, &tts_context).await
}
