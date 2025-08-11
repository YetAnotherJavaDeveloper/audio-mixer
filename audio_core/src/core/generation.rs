use std::f32::consts::PI;
use super::models::{Sample, Rate};

pub fn generate_sine_wave(samples: usize, frequency: f32, sample_rate: &Rate) -> Sample {
    let mut wave = Vec::with_capacity(samples);
    for i in 0..samples {
        let sample = (i as f32 * frequency * 2.0 * PI / sample_rate.value() as f32).sin();
        wave.push(sample);
    }
    Sample::new(wave)
}