use std::f64::consts::PI;

pub fn sine_sample(sample_number: u32, frequency: f64, sample_rate: u32) -> f64 {
    let time = sample_number as f64 / sample_rate as f64;
    let angle = 2.0 * PI * frequency * time;
    let sample = angle.sin();
    return sample;
}
