use crate::core::{MusicSample, Sample};

pub fn print_sample_info(sample: &Sample) {
    println!("Sample length: {}", sample.len());
    println!("Min and max {:?}", find_minmax_sample(&sample))
}

pub fn print_music_sample_info(music_sample: &MusicSample) {
    let num_samples = music_sample.multi_channel_sample().channels();
    let sample_length = music_sample.first_channel_sample().len();
    let sample_rate = music_sample.sample_rate().value();
    let channels = music_sample.channels();

    println!("Sample info : ");
    println!("Number of samples: {}", num_samples);
    println!("Sample length: {}", sample_length);
    println!("Sample rate: {} Hz", sample_rate);
    println!("Number of channels: {}", channels);

    let duration = sample_length as f64 / sample_rate as f64;
    println!("Duration: {:.2} seconds", duration);

    println!("Min and max : {:?}", find_minmax_music_sample(&music_sample));
    println!();
}

fn find_minmax_music_sample(sample: &MusicSample) -> (f32, f32) {
    let mut min = 0.0;
    let mut max = 0.0;

    let mut sample_min = 0.0;
    let mut sample_max = 0.0;

    for sample in sample.multi_channel_sample().samples() {
        (sample_min, sample_max) = find_minmax_sample(sample);
        if sample_min < min {
            min = sample_min;
        }
        if(sample_max > max) {
            max = sample_max;
        }
    }

    (min, max)
}

fn find_minmax_sample(sample: &Sample) -> (f32, f32) {
    let mut min = 0.0;
    let mut max = 0.0;

    for y in sample.values() {
        if y < &min {
            min = *y;
        }
        if y > &max {
            max = *y;
        }
    }

    (min, max)
}