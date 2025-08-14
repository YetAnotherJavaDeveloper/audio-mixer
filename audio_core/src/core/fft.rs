#![allow(dead_code)]

use super::models::{FftDefinition, Rate, Sample};
use crate::core::FftResult;
use rustfft::FftPlanner;
use rustfft::num_complex::{Complex32, ComplexFloat};

pub fn test() -> String {
    return "This is a test function in fft.rs".to_string();
}

pub fn fft_to_frequencies(
    sample: &Sample,
    rate: &Rate,
    fft_definition: &FftDefinition,
) -> Result<Vec<FftResult>, String> {
    let fft = FftPlanner::new().plan_fft(sample.len(), rustfft::FftDirection::Forward);

    let mut complex_output: Vec<Complex32> = sample
        .values()
        .iter()
        .map(|&x| Complex32::new(x, 0.0))
        .collect();
    fft.process(&mut complex_output);

    let length = sample.len();

    let min_index = fft_definition.start_frequency() as f32 * length as f32 / rate.value() as f32;
    let max_index = fft_definition.end_frequency() as f32 * length as f32 / rate.value() as f32;

    let mut result: Vec<FftResult> = Vec::new();

    let mut base_frequency = fft_definition.start_frequency() as f32;
    let mut next_frequency = base_frequency + fft_definition.frequency_precision() as f32;

    let mut max_magnitude = f32::MIN;
    let mut max_magnitude_frequency = 0.0;

    for i in min_index as usize..max_index as usize {
        let magnitude = complex_output[i].abs();
        let frequency = i as f32 * rate.value() as f32 / length as f32;

        if frequency > next_frequency || i == max_index as usize {
            result.push(FftResult::new(
                base_frequency,
                max_magnitude_frequency,
                max_magnitude,
            ));
            max_magnitude = f32::MIN;
            base_frequency += fft_definition.frequency_precision() as f32;
            next_frequency += fft_definition.frequency_precision() as f32;
        }
        if magnitude > max_magnitude {
            max_magnitude = magnitude;
            max_magnitude_frequency = frequency;
        }
    }

    Ok(result)
}
