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
        MultiChannelSample {
            samples: Vec::new(),
        }
    }

    pub fn new(samples: Vec<Sample>) -> MultiChannelSample {
        MultiChannelSample { samples }
    }

    pub fn with_capacity(capacity: usize) -> MultiChannelSample {
        MultiChannelSample {
            samples: vec![Sample::empty(); capacity],
        }
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
pub struct Rate {
    value: u32,
}

impl Rate {
    pub fn new(value: u32) -> Rate {
        Rate { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn convert_pos_to_time_ms(&self, pos: usize) -> usize {
        ((pos * 1000) as f32 / self.value as f32).floor() as usize
    }

    pub fn convert_time_ms_to_pos(&self, time_ms: usize) -> usize {
        (time_ms as f32 * self.value as f32 / 1000.0).floor() as usize
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MusicSample {
    multi_channel_sample: MultiChannelSample,
    sample_rate: Rate,
}

impl MusicSample {

    pub fn new(multi_channel_sample: MultiChannelSample, sample_rate: Rate) -> MusicSample {
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

    pub fn sample_rate(&self) -> &Rate {
        &self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.multi_channel_sample.channels()
    }

    pub fn first_channel_sample(&self) -> &Sample {
        self.multi_channel_sample.first_channel()
    }
}

pub struct MusicTime {
    pos: usize,
    rate: Rate,
}

impl MusicTime {

    pub fn from_pos(pos: usize, rate: &Rate) -> MusicTime {
        MusicTime { pos, rate: rate.clone() }
    }

    pub fn from_time_ms(time: usize, rate: &Rate) -> MusicTime {
        MusicTime {
            pos : rate.convert_time_ms_to_pos(time),
            rate: rate.clone(),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn rate(&self) -> &Rate {
        &self.rate
    }

    pub fn time_ms(&self) -> usize {
        self.rate.convert_pos_to_time_ms(self.pos)
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
