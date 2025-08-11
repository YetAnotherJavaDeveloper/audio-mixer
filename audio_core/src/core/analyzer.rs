use crate::core::MusicSample;

pub fn extract_sample_info(sample: &MusicSample) {
    let num_samples = sample.multi_channel_sample().channels();
    let sample_length = sample.first_channel_sample().len();
    let sample_rate = sample.sample_rate().value();
    let channels = sample.channels();

    println!("Sample info : ");
    println!("Number of samples: {}", num_samples);
    println!("Sample length: {}", sample_length);
    println!("Sample rate: {} Hz", sample_rate);
    println!("Number of channels: {}", channels);

    let duration = sample_length as f64 / sample_rate as f64;
    println!("Duration: {:.2} seconds", duration);

    find_minmax(&sample);
    println!();
}

fn find_minmax(sample: &MusicSample) {
    let mut min = 0.0;
    let mut max = 0.0;

    for x in sample.multi_channel_sample().samples() {
        for y in x.values() {
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