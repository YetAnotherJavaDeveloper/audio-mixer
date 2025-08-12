#![allow(dead_code)]
use std::fs::File;
use rodio::{ChannelCount, Decoder, OutputStream, Sample, Sink};
use rodio::buffer::SamplesBuffer;
use crate::core::MusicSample;

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
    music_sample: MusicSample,
    pub sink: Sink,

    #[allow(dead_code)]
    output_stream: OutputStream,
    first_play: bool,
    pub current_position: usize,
}

impl MusicSamplesPlayer {
    pub fn new(music_sample: MusicSample) -> MusicSamplesPlayer {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        MusicSamplesPlayer {
            music_sample,
            sink: Sink::connect_new(&stream_handle.mixer()),
            output_stream: stream_handle,
            first_play: true,
            current_position: 0,
        }
    }

    pub fn play(&mut self) {

        if self.first_play {

            let mut buffer_data : Vec<Sample> = Vec::new();

            for pos in 0..self.music_sample.first_channel_sample().len() {
                self.current_position = pos;
                for ch in 0..self.music_sample.multi_channel_sample().channels() {
                    buffer_data.push(self.music_sample.multi_channel_sample().sample(ch).value(pos));
                }
            }

            let samples_buffer = SamplesBuffer::new(
                ChannelCount::from(self.music_sample.channels() as u16),
                self.music_sample.sample_rate().value(),
                buffer_data
            );

            self.sink.append(samples_buffer);
            self.first_play = false;
        } else {
            self.sink.play();
        }
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }
}