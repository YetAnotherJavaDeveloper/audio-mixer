#![allow(dead_code)]

use crate::core::{MultiChannelSample, MusicSample, MusicTime};

pub fn split_music_samples(music_sample: &MusicSample, music_time: MusicTime) -> (MusicSample, MusicSample) {

    let split_samples = split_samples(&music_sample.multi_channel_sample(), music_time);

    (music_sample.copy(split_samples.0), music_sample.copy(split_samples.1))
}

pub fn split_samples(multi_channel_sample: &MultiChannelSample, music_time: MusicTime) -> (MultiChannelSample, MultiChannelSample) {

    if multi_channel_sample.channels() == 0 || multi_channel_sample.first_channel().len() < music_time.pos() {
        panic!("Can not split")
    }

    let mut split_samples: (MultiChannelSample, MultiChannelSample) = (MultiChannelSample::empty(), MultiChannelSample::empty());

    for samples in multi_channel_sample.samples() {
        let s = samples.split_at(music_time.pos());
        split_samples.0.push(s.0);
        split_samples.1.push(s.1);
    }

    split_samples
}