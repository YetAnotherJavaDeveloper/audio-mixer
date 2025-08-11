use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sample {
    values: Vec<f32>,
}

impl Sample {

    pub fn empty() -> Sample {
        Sample { values: Vec::new() }
    }

    pub fn new(values: Vec<f32>) -> Sample {
        Sample { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn values(&self) -> &Vec<f32> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<f32> {
        &mut self.values
    }

    pub fn value(&self, index: usize) -> f32 {
        self.values[index]
    }

    pub fn push(&mut self, value: f32) {
        self.values.push(value);
    }

    pub fn split_at(&self, mid: usize) -> (Sample, Sample) {
        let s = self.values().split_at(mid);

        (Sample::new(s.0.to_vec()), Sample::new(s.1.to_vec()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiChannelSample {
    samples: Vec<Sample>,
}

impl MultiChannelSample {
    pub fn empty() -> MultiChannelSample {
        MultiChannelSample { samples: Vec::new() }
    }

    pub fn new(samples: Vec<Sample>) -> MultiChannelSample {
        MultiChannelSample { samples }
    }
    
    pub fn with_capacity(capacity: usize) -> MultiChannelSample {
        MultiChannelSample { samples: vec![Sample::empty(); capacity] }
    }

    pub fn channels(&self) -> usize {
        self.samples.len()
    }

    pub fn samples(&self) -> &Vec<Sample> {
        &self.samples
    }

    pub fn samples_mut(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }

    pub fn sample(&self, index: usize) -> &Sample {
        &self.samples[index]
    }

    pub fn sample_mut(&mut self, index: usize) -> &mut Sample {
        &mut self.samples[index]
    }

    pub fn first_channel(&self) -> &Sample {
        self.samples.first().unwrap()
    }

    pub fn push(&mut self, sample: Sample) {
        self.samples.push(sample);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SampleRate {
    value: u32,
}

impl SampleRate {
    pub fn new(value: u32) -> SampleRate {
        SampleRate { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MusicSample {
    multi_channel_sample: MultiChannelSample,
    sample_rate: SampleRate,
}

impl MusicSample {

    pub fn new(multi_channel_sample: MultiChannelSample, sample_rate: SampleRate) -> MusicSample {
        MusicSample {
            multi_channel_sample,
            sample_rate,
        }
    }

    pub fn copy(&self, multi_channel_sample: MultiChannelSample) -> MusicSample {
        MusicSample {
            multi_channel_sample,
            sample_rate: self.sample_rate.clone(),
        }
    }

    pub fn multi_channel_sample(&self) -> &MultiChannelSample {
        &self.multi_channel_sample
    }

    pub fn sample_rate(&self) -> &SampleRate {
        &self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.multi_channel_sample.channels()
    }

    pub fn first_channel_sample(&self) -> &Sample {
        self.multi_channel_sample.first_channel()
    }
}

pub enum Transformation {
    Reverse,
    Flat,
    SpeedChelou,
    DoubleSpeed,
    Echo,
    DoubleLeft,
}
