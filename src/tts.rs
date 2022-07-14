const SAMPLE_RATE: f64 = 24000.0;

const CHANNELS: i32 = 1;

const FRAMES: u32 = 1024;

async fn speak(msg: &str, audio_device: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let synthesis_query = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", msg), ("speaker", "0")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let wav_bytes = client
        .post("http://localhost:50021/synthesis")
        .query(&[("speaker", "0")])
        .json(&synthesis_query)
        .send()
        .await?
        .bytes()
        .await?;

    let pa = portaudio::PortAudio::new()?;

    let (device_index, device_info) = pa
        .devices()?
        .filter_map(|device| device.ok())
        .find(|(_, device_info)| device_info.name == audio_device)
        .expect(format!("`{}` device not found", audio_device).as_str());

    let output_params = portaudio::StreamParameters::<f32>::new(
        device_index,
        CHANNELS,
        true,
        device_info.default_low_input_latency,
    );

    pa.is_output_format_supported(output_params, SAMPLE_RATE)?;

    let output_settings = portaudio::OutputStreamSettings::new(output_params, SAMPLE_RATE, FRAMES);

    let mut stream = pa.open_blocking_stream(output_settings)?;

    let mut reader = hound::WavReader::new(std::io::Cursor::new(wav_bytes))?;
    let wav_buffer: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let mut wav_buffer_iter = wav_buffer.iter();

    stream.start()?;

    let n_write_samples = FRAMES as usize * CHANNELS as usize;
    let mut completed = false;
    while !completed {
        stream.write(FRAMES as u32, |output| {
            for i in 0..n_write_samples {
                if let Some(t) = wav_buffer_iter.next() {
                    output[i] = 0.3 * (*t as f32 / 32767.0);
                } else {
                    completed = true;
                }
            }
        })?;
    }

    stream.close()?;

    Ok(())
}

pub async fn speak_each_tweet(mut recv: tokio::sync::mpsc::Receiver<String>, audio_device: String) {
    loop {
        if let Ok(msg) = recv.try_recv() {
            if let Err(err) = speak(&msg, &audio_device).await {
                eprintln!("{}", err)
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
