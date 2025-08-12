mod file;
mod core;
mod media;

use audio_core::core::{generate_sine_wave, fft_to_frequencies, FftDefinition, print_fft_result};
use audio_core::core::{print_music_sample_info, split_music_samples, transform_abstract, MusicSample, MusicTime};
use audio_core::file::{read_music_samples_from_file};
use audio_core::media::{MusicSamplesPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let music_sample = read_music_samples_from_file(String::from("input.mp3"))?;

    print_music_sample_info(&music_sample);

    let processed: MusicSample = transform_abstract(&music_sample);

    let split_time = MusicTime::from_time_ms(7_000, music_sample.sample_rate());

    let processed = split_music_samples(&processed, split_time).1;

    let sin_wave = generate_sine_wave(65_636, 880.0, music_sample.sample_rate());

    let fft_definition = FftDefinition::for_frequency_precision(100);

    let frequencies = fft_to_frequencies(&sin_wave, music_sample.sample_rate(), &fft_definition);

    print_fft_result(&frequencies, &fft_definition);

    print_music_sample_info(&processed);

    let mut player = MusicSamplesPlayer::new(processed);
    player.play();
    player.sink.sleep_until_end();

    Ok(())
}

