use anyhow::Context;
use log::trace;
use std::{io, thread, time};

pub struct TTSContext {
    pub pa: portaudio::PortAudio,
    pub output_settings: portaudio::OutputStreamSettings<f32>,
}

pub const SAMPLE_RATE: f64 = 24000.0;

pub const CHANNELS: i32 = 1;

pub const FRAMES: u32 = 1024;

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
    context: &TTSContext,
) -> anyhow::Result<()> {
    loop {
        if let Ok(msg) = recv.try_recv() {
            if let Err(err) = speak(&msg, context).await {
                eprintln!("{}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}
