mod file;
mod core;
mod media;
mod app;

use iced::Task;

use crate::core::{find_minmax, transform_abstract, MusicSamples};
use crate::file::{read_music_samples_from_file, save_music_samples};
use crate::media::{MediaFilePlayer, MusicSamplesPlayer};

use crate::app::AudioMixer;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let music_samples = read_music_samples_from_file(String::from("input.mp3"))?;

    // find_minmax(&music_samples);

    // let processed: MusicSamples = transform_abstract(&music_samples);

    // find_minmax(&processed);

    //MusicSamplesPlayer::new(processed).play();

    // MediaFilePlayer::new(String::from("output.wav")).play();
    iced::application("AudioMixer", AudioMixer::update, AudioMixer::view)
        .run_with(|| (AudioMixer::new(), Task::none()));

    Ok(())
}

