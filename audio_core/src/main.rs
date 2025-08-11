mod file;
mod core;
mod media;

use audio_core::core::{fft_to_frequency, generate_sine_wave};
use audio_core::core::{print_music_sample_info, split_music_samples, transform_abstract, MultiChannelSample, MusicSample, MusicTime};
use audio_core::file::{read_music_samples_from_file, save_music_samples};
use audio_core::media::{MediaFilePlayer, MusicSamplesPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let music_sample = read_music_samples_from_file(String::from("input.mp3"))?;

    print_music_sample_info(&music_sample);

    let processed: MusicSample = transform_abstract(&music_sample);

    let split_time = MusicTime::from_time_ms(7_000, music_sample.sample_rate());

    let processed = split_music_samples(&processed, split_time).1;

    let sin_wave = generate_sine_wave(100_000, 880.0, processed.sample_rate());

    println!("Frequency : {}", fft_to_frequency(&sin_wave, processed.sample_rate()));

    let processed = processed.copy(MultiChannelSample::mono(sin_wave));

    print_music_sample_info(&processed);

    let mut player = MusicSamplesPlayer::new(processed);
    player.play();
    player.sink.sleep_until_end();

    Ok(())
}

