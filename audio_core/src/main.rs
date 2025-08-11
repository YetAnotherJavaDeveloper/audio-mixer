mod file;
mod core;
mod media;

use crate::core::{extract_sample_info, split_music_samples, transform_abstract, MusicSample, MusicTime};
use crate::file::{read_music_samples_from_file, save_music_samples};
use crate::media::{MediaFilePlayer, MusicSamplesPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let music_sample = read_music_samples_from_file(String::from("input.mp3"))?;

    extract_sample_info(&music_sample);

    let processed: MusicSample = transform_abstract(&music_sample);

    let split_time = MusicTime::from_time_ms(7_000, music_sample.sample_rate());

    let processed = split_music_samples(&processed, split_time).1;

    extract_sample_info(&processed);

    let mut player = MusicSamplesPlayer::new(processed);
    player.play();
    player.sink.sleep_until_end();

    Ok(())
}

