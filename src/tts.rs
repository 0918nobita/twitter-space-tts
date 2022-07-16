use anyhow::Context;
use log::trace;
use std::{io, thread, time};

pub struct TTSConfig {
    pub audio_output_device: Option<String>,
}

struct TTSContext {
    pa: portaudio::PortAudio,
    output_settings: portaudio::OutputStreamSettings<f32>,
}

const SAMPLE_RATE: f64 = 24000.0;

const CHANNELS: i32 = 1;

const FRAMES: u32 = 1024;

async fn speak(msg: &str, context: &TTSContext) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    trace!("Creating synthesis query");

    let synthesis_query = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", msg), ("speaker", "0")])
        .send()
        .await
        .context("Faild to send HTTP request to VOICEVOX engine")?
        .json::<serde_json::Value>()
        .await
        .context("Failed to parse response from VOICEVOX engine as JSON")?;

    trace!("Acuquiring the result of sound synthesis (wav)");

    let wav_bytes = client
        .post("http://localhost:50021/synthesis")
        .query(&[("speaker", "0")])
        .json(&synthesis_query)
        .send()
        .await
        .context("Failed to send HTTP request to VOICEVOX engine")?
        .bytes()
        .await
        .context("Failed to parse response from VOICEVOX engine as bytes")?;

    trace!("Creating audio output stream");
    let mut stream = context.pa.open_blocking_stream(context.output_settings)?;

    trace!("Preparing wave file loader and buffer");
    let mut reader = hound::WavReader::new(io::Cursor::new(wav_bytes))?;
    let wav_buffer: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let mut wav_buffer_iter = wav_buffer.iter();

    trace!("Starting audio output stream");
    stream.start()?;

    let n_write_samples = FRAMES as usize * CHANNELS as usize;
    let mut completed = false;
    while !completed {
        stream.write(FRAMES as u32, |output| {
            for out in output.iter_mut().take(n_write_samples) {
                if let Some(t) = wav_buffer_iter.next() {
                    *out = 0.08 * (*t as f32 / 32767.0);
                } else {
                    completed = true;
                }
            }
        })?;
    }

    trace!("Closing audio output stream");
    stream.close()?;

    Ok(())
}

pub async fn speak_each_tweet(
    mut recv: tokio::sync::mpsc::Receiver<String>,
    config: &TTSConfig,
) -> anyhow::Result<()> {
    let pa = portaudio::PortAudio::new().context("Failed to initialize PortAudio")?;
    let (device_index, device_info) = if let Some(device_name) = &config.audio_output_device {
        pa.devices()
            .context("Failed to enumerate audio devices")?
            .filter_map(|device| device.ok())
            .find(|(_, device_info)| device_info.name == device_name)
            .with_context(|| format!("`{}` device not found", device_name))?
    } else {
        let device_index = pa.default_output_device()?;
        let device_info = pa.device_info(device_index)?;
        (device_index, device_info)
    };

    let output_params = portaudio::StreamParameters::<f32>::new(
        device_index,
        CHANNELS,
        true,
        device_info.default_low_input_latency,
    );

    pa.is_output_format_supported(output_params, SAMPLE_RATE)?;

    let output_settings = portaudio::OutputStreamSettings::new(output_params, SAMPLE_RATE, FRAMES);

    let context = TTSContext {
        pa,
        output_settings,
    };

    loop {
        if let Ok(msg) = recv.try_recv() {
            if let Err(err) = speak(&msg, &context).await {
                eprintln!("{}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}
