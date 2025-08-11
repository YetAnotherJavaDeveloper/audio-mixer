mod file;
mod core;
mod media;

use crate::core::{extract_sample_info, transform_abstract, MusicSamples};
use crate::file::{read_music_samples_from_file, save_music_samples};
use crate::media::{MediaFilePlayer, MusicSamplesPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let music_samples = read_music_samples_from_file(String::from("input.mp3"))?;

    extract_sample_info(&music_samples);

    let processed: MusicSamples = transform_abstract(&music_samples);

    extract_sample_info(&processed);
    
    let mut player = MusicSamplesPlayer::new(processed);
    player.play();
    player.sink.sleep_until_end();

    Ok(())
}

