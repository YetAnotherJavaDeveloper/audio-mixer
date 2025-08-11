use crate::core::MusicSamples;

pub fn extract_sample_info(samples: &MusicSamples) {
    let num_samples = samples.all_samples.len();
    let sample_length = samples.all_samples[0].len();
    let sample_rate = samples.sample_rate;
    let channels = samples.channels;

    println!("Sample info : ");
    println!("Number of samples: {}", num_samples);
    println!("Sample length: {}", sample_length);
    println!("Sample rate: {} Hz", sample_rate);
    println!("Number of channels: {}", channels);

    let duration = sample_length as f64 / sample_rate as f64;
    println!("Duration: {:.2} seconds", duration);

    find_minmax(&samples);
    println!();
}

fn find_minmax(samples: &MusicSamples) {
    let mut min = 0.0;
    let mut max = 0.0;

    for x in &samples.all_samples {
        for y in x {
            if y < &min {
                min = *y;
            }
            if y > &max {
                max = *y;
            }
        }
    }

    println!("Sample min: {}", min);
    println!("Sample max: {}", max);
}