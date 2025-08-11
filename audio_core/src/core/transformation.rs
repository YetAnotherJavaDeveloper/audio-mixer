use crate::core::models::{Sample, Transformation};
use crate::core::{MultiChannelSample, MusicSample};

pub fn transform_abstract(sample: &MusicSample) -> MusicSample {
    sample.copy(transform_generic(sample.multi_channel_sample(), Transformation::Reverse))
}

fn transform_generic(multi_channel_sample: &MultiChannelSample, transformation: Transformation) -> MultiChannelSample {

    match transformation {
        Transformation::Reverse => {transform_reverse(&multi_channel_sample)}
        Transformation::Flat => {transform_flat(&multi_channel_sample)}
        Transformation::SpeedChelou => {transform_speed_chelou(&multi_channel_sample)}
        Transformation::DoubleSpeed => {transform_double_speed(&multi_channel_sample)}
        Transformation::Echo => {transform_echo(&multi_channel_sample)}
        Transformation::DoubleLeft => {transform_double_left(&multi_channel_sample)}
    }
}

fn transform_reverse(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {
    let mut result: MultiChannelSample = MultiChannelSample::empty();
    for channel in 0..multi_channel_sample.channels() {
        let mut result_sample  = Sample::empty();

        let mut i = multi_channel_sample.sample(channel).len();
        while i > 0 {
            result_sample.push(multi_channel_sample.sample(channel).value(i - 1));
            i -= 1;
        }


        result.push(result_sample);
    }

    result
}

fn transform_flat(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {
    let mut result: MultiChannelSample = MultiChannelSample::empty();
    for channel in 0..multi_channel_sample.channels() {
        let mut result_sample: Sample = Sample::empty();

        let mut i: usize = 0;
        while i < multi_channel_sample.sample(channel).len() {

            if i < 10 {
                result_sample.push(multi_channel_sample.sample(channel).value(i));
            } else {
                let mut value = multi_channel_sample.sample(channel).value(i);
                for j in 0 .. 10  {
                    value += multi_channel_sample.sample(channel).value(i - j);
                }
                result_sample.push(value / 10.0);
            }
            i += 1;
        }


        result.push(result_sample);
    }

    result
}


enum MusicMode {
    HALF,
    NORMAL,
    DOUBLE
}

pub fn transform_speed_chelou(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {
    let mut result: MultiChannelSample = MultiChannelSample::empty();

    for channel in 0..multi_channel_sample.channels() {
        let mut result_sample: Sample = Sample::empty();

        let time_each_mode= 22000;
        let mut music_mode = MusicMode::NORMAL;
        let mut music_mode_incr = 0;

        let mut i = 0;
        while i < multi_channel_sample.sample(channel).len() {
            match music_mode {
                MusicMode::HALF => {
                    if i % 2 == 0 {
                        result_sample.push(multi_channel_sample.sample(channel).value(i));
                    } else {
                        result_sample.push(multi_channel_sample.sample(channel).value(i - 1));
                    }
                }
                MusicMode::NORMAL => {
                    result_sample.push(multi_channel_sample.sample(channel).value(i));
                }
                MusicMode::DOUBLE => {
                    if i % 2 == 0 {
                        result_sample.push(multi_channel_sample.sample(channel).value(i));
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

        result.push(result_sample);
    }

    result
}

pub fn transform_double_speed(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {
    let mut result: MultiChannelSample = MultiChannelSample::empty();

    for channel in 0..multi_channel_sample.channels() {
        let mut result_sample: Sample = Sample::empty();

        let mut i = 0;
        while i < multi_channel_sample.sample(channel).len() {
            result_sample.push(multi_channel_sample.sample(channel).value(i));
            i = i + 2;
        }

        result.push(result_sample);
    }

    result
}

pub fn transform_echo(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {

    let time_delay_sample = 22000;

    let mut result: MultiChannelSample = MultiChannelSample::empty();

    for channel in 0..multi_channel_sample.channels() {

        let mut delay_before: Sample = Sample::empty();
        let mut delay_after: Sample = Sample::empty();
        let mut result_vec: Sample = Sample::empty();

        let mut i = 0;
        while i < time_delay_sample {
            delay_before.push(0.0);
            i = i + 1;
        }

        for x in multi_channel_sample.sample(channel).values() {
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
            result_vec.push(delay_after.value(i) + delay_before.value(i));
            i = i + 1;
        }


        result.push(result_vec);
    }



    result
}

pub fn transform_double_left(multi_channel_sample: &MultiChannelSample) -> MultiChannelSample {

    let mut left: Sample = Sample::empty();
    let mut right: Sample = Sample::empty();

    for x in multi_channel_sample.first_channel().values() {
        left.push(*x);
        left.push(*x);
    }

    for x in multi_channel_sample.samples()[1].values() {
        right.push(*x);
    }
    for x in multi_channel_sample.samples()[1].values() {
        right.push(*x);
    }

    let mut result: MultiChannelSample = MultiChannelSample::empty();
    result.push(left);
    result.push(right);

    result
}