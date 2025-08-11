use crate::core::MusicSamples;
use crate::media::MusicSamplesPlayer;
pub struct AudioMixer {
    pub is_loading: bool,
    pub is_loaded: bool,
    pub is_playing: bool,
    pub length: f32,
    pub current_position: usize,
    // optional file path
    pub file_path: String,
    pub loaded_samples: Option<MusicSamples>,
    pub media_player: Option<MusicSamplesPlayer>,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenFileDialog,
    FileChosen(String),
    FileLoaded(MusicSamples),
    PlayPause,
    Error(String),
    Tick
}