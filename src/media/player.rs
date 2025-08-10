use std::fs::File;
use rodio::{ChannelCount, Decoder, Sample, Sink};
use rodio::buffer::SamplesBuffer;
use crate::core::MusicSamples;

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

pub struct MusicSamplesPlayer {
    music_samples: MusicSamples
}

impl MusicSamplesPlayer {
    pub fn new(music_samples: MusicSamples) -> MusicSamplesPlayer {
        MusicSamplesPlayer {
            music_samples: music_samples
        }
    }

    pub fn play(&self) {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");

        let sink = Sink::connect_new(&stream_handle.mixer());

        let mut buffer_data : Vec<Sample> = Vec::new();

        for pos in 0..self.music_samples.all_samples[0].len() {
            for ch in 0..self.music_samples.all_samples.len() {
                buffer_data.push(self.music_samples.all_samples[ch][pos]);
            }
        }

        let samples_buffer = SamplesBuffer::new(
            ChannelCount::from(self.music_samples.channels as u16),
            self.music_samples.sample_rate,
            buffer_data
        );

        sink.append(samples_buffer);

        sink.sleep_until_end();
    }
}