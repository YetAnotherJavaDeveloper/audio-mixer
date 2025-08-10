use std::fs::File;
use rodio::{Decoder, Sink};

pub struct MediaFilePlayer {
    file_path: String,
}

impl MediaFilePlayer {
    pub fn new(file_path: String) -> MediaFilePlayer {
        MediaFilePlayer { file_path }
    }
    
    pub fn play(&self) {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");

        let sink = Sink::connect_new(&stream_handle.mixer());

        let file = File::open(&self.file_path).unwrap();

        let source = Decoder::try_from(file).unwrap();

        sink.append(source);

        sink.sleep_until_end();
    }
}