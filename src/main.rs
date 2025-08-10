mod file;
mod core;
mod media;

use crate::core::{find_minmax, transform_abstract, MusicSamples};
use crate::file::{read_music_samples_from_file, save_music_samples};
use crate::media::{MediaFilePlayer, MusicSamplesPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let music_samples = read_music_samples_from_file(String::from("input.mp3"))?;

    find_minmax(&music_samples);

    let processed: MusicSamples = transform_abstract(&music_samples);

    find_minmax(&processed);

    save_music_samples(&processed, "output.wav")?;

    MusicSamplesPlayer::new(processed).play();

    //MediaFilePlayer::new(String::from("output.wav")).play();

    Ok(())
}

