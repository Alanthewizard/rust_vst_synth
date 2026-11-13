mod oscillator;

fn main() {
    let sample_rate: u32 = 44100;
    let seconds: u32 = 3;
    let frequency = 440.0;
    let volume = 0.15;
    let sample_count = sample_rate * seconds;
    let mut samples: Vec<i16> = Vec::new();

    for sample_number in 0..sample_count {
        let sample = oscillator::sine_sample(sample_number, frequency, sample_rate);
        let quiet_sample = sample * volume;
        let saved_sample = (quiet_sample * 32767.0) as i16;
        samples.push(saved_sample);
    }
    save_wav(samples, sample_rate);
    println!("Created sine.wav. Open it in an audio player to hear the sine wave!");
}

fn save_wav(samples: Vec<i16>, sample_rate: u32) {
    let mut bytes: Vec<u8> = Vec::new();
    let data_size = samples.len() as u32 * 2;
    let file_size = 36 + data_size;
    let bytes_per_second = sample_rate * 2;

    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&file_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&bytes_per_second.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());

    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    std::fs::write("sine.wav", &bytes).expect("Could not save sine.wav");
}
