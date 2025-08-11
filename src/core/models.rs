#[derive(Clone, Debug)]
pub struct MusicSamples {
    pub all_samples: Vec<Vec<f32>>,
    pub sample_rate: u32,
    pub channels: usize
}

pub enum Transformation {
    Reverse,
    Flat,
    SpeedChelou,
    DoubleSpeed,
    Echo,
    DoubleLeft,
    DoNothing
}