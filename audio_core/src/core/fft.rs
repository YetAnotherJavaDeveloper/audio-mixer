use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::{Complex, ComplexFloat};
use super::models::{Sample, Rate};

pub fn fft_to_frequency(sample: &Sample, rate: &Rate) -> f32 {

    let fft = FftPlanner::new().plan_fft(sample.len(), rustfft::FftDirection::Forward);

    let mut complex_output: Vec<Complex<f32>> = sample.values()
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    fft.process(&mut complex_output);

    let length = sample.len();

    let mut max_idx = 0;
    let mut magnitude = f32::MIN;
    for i in 0..length / 2 {
        let val = complex_output[i].abs();
        if val > magnitude {
            magnitude = val;
            max_idx = i;
        }
    }

    let frequency = max_idx as f32 * rate.value() as f32 / length as f32;
    frequency
}