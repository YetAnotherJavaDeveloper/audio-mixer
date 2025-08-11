use crate::core::models::Transformation;
use crate::core::MusicSamples;

pub fn transform_abstract(samples: &MusicSamples) -> MusicSamples {
    MusicSamples{
        all_samples: transform_generic(&samples.all_samples, Transformation::Reverse),
        sample_rate: samples.sample_rate,
        channels: samples.channels,
    }
}

fn transform_generic(samples: &Vec<Vec<f32>>, transformation: Transformation) -> Vec<Vec<f32>> {

    match transformation {
        Transformation::Reverse => {transform_reverse(&samples)}
        Transformation::Flat => {transform_flat(&samples)}
        Transformation::SpeedChelou => {transform_speed_chelou(&samples)}
        Transformation::DoubleSpeed => {transform_double_speed(&samples)}
        Transformation::Echo => {transform_echo(&samples)}
        Transformation::DoubleLeft => {transform_double_left(&samples)}
        Transformation::DoNothing => {transform_do_nothing(&samples)}
    }
}

fn transform_reverse(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    for lr in 0..2 {
        let mut result_vec: Vec<f32>  = Vec::new();

        let mut i = samples[lr].len();
        while i > 0 {
            result_vec.push(samples[lr][i -1]);
            i -= 1;
        }


        result.push(result_vec);
    }

    result
}

fn transform_flat(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    for lr in 0..2 {
        let mut result_vec: Vec<f32>  = Vec::new();

        let mut i: usize = 0;
        while i < samples[lr].len() {

            if i < 10 {
                result_vec.push(samples[lr][i]);
            } else {
                let mut value = samples[lr][i];
                for j in 0 .. 10  {
                    value += samples[lr][i - j];
                }
                result_vec.push(value / 10.0);
            }
            i += 1;
        }


        result.push(result_vec);
    }

    result
}


enum MusicMode {
    HALF,
    NORMAL,
    DOUBLE
}

pub fn transform_speed_chelou(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();

    for lr in 0..2 {
        let mut result_vec: Vec<f32>  = Vec::new();

        let time_each_mode= 22000;
        let mut music_mode = MusicMode::NORMAL;
        let mut music_mode_incr = 0;

        let mut i = 0;
        while i < samples[lr].len() {
            match music_mode {
                MusicMode::HALF => {
                    if i % 2 == 0 {
                        result_vec.push(samples[lr][i]);
                    } else {
                        result_vec.push(samples[lr][i - 1]);
                    }
                }
                MusicMode::NORMAL => {
                    result_vec.push(samples[lr][i]);
                }
                MusicMode::DOUBLE => {
                    if i % 2 == 0 {
                        result_vec.push(samples[lr][i]);
                    }
                }
            }
            i = i + 1;
            music_mode_incr += 1;

            if music_mode_incr > time_each_mode {
                match music_mode {
                    MusicMode::HALF => {
                        music_mode = MusicMode::NORMAL;
                    }
                    MusicMode::NORMAL => {
                        music_mode = MusicMode::DOUBLE;
                    }
                    MusicMode::DOUBLE => {
                        music_mode = MusicMode::HALF;
                    }
                }
                music_mode_incr = 0;
            }
        }

        result.push(result_vec);
    }

    result
}

pub fn transform_double_speed(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();

    for lr in 0..2 {
        let mut result_vec: Vec<f32>  = Vec::new();

        let mut i = 0;
        while i < samples[lr].len() {
            result_vec.push(samples[lr][i]);
            i = i + 2;
        }

        result.push(result_vec);
    }

    result
}

pub fn transform_echo(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {

    let time_delay_sample = 22000;

    let mut result: Vec<Vec<f32>> = Vec::new();

    for lr in 0..2 {

        let mut delay_before: Vec<f32>  = Vec::new();
        let mut delay_after: Vec<f32>  = Vec::new();
        let mut result_vec: Vec<f32>  = Vec::new();

        let mut i = 0;
        while i < time_delay_sample {
            delay_before.push(0.0);
            i = i + 1;
        }

        for x in &samples[lr] {
            delay_before.push(*x);
            delay_after.push(*x);
        }

        let mut i = 0;
        while i < time_delay_sample {
            delay_after.push(0.0);
            i = i + 1;
        }

        let mut i = 0;
        while i < delay_before.len() {
            result_vec.push(delay_after[i] + delay_before[i]);
            i = i + 1;
        }


        result.push(result_vec);
    }



    result
}

pub fn transform_double_left(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {

    let mut left: Vec<f32>  = Vec::new();
    let mut right: Vec<f32>  = Vec::new();

    for x in &samples[0] {
        left.push(*x);
        left.push(*x);
    }

    for x in &samples[1] {
        right.push(*x);
    }
    for x in &samples[1] {
        right.push(*x);
    }

    let mut result: Vec<Vec<f32>> = Vec::new();
    result.push(left);
    result.push(right);

    result
}

pub fn transform_do_nothing(samples: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    //simple do nothing transformation
    samples
        .iter()
        .map(|v| v.iter().map(|&s| s * 1.0).collect::<Vec<_>>())//.clamp(-1.0, 1.0))
        .collect()
}