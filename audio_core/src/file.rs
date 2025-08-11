use std::fs::File;
use rodio::{Decoder, Sink};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::default::{get_codecs, get_probe};
use super::core::MusicSamples;

pub fn read_music_samples_from_file(file_path: String) -> Result<MusicSamples, Box<dyn std::error::Error>> {

    // --- 1. Open and decode audio with Symphonia ---
    let file = File::open(file_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let mut format = probed.format;
    let track = format.default_track().ok_or("No default track found")?;

    let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let mut channels_data: Vec<Vec<f32>> = vec![Vec::new(); track.codec_params.channels.unwrap().count()];
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.unwrap().count();

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet)?;

        match decoded {
            AudioBufferRef::F32(buf) => {
                for ch in 0..buf.spec().channels.count() {
                    channels_data[ch].extend_from_slice(buf.chan(ch));
                }
            }
            AudioBufferRef::S16(buf) => {
                for ch in 0..buf.spec().channels.count() {
                    channels_data[ch].extend(buf.chan(ch).iter().map(|&s| s as f32 / i16::MAX as f32));
                }
            }
            _ => return Err("Unsupported sample format".into()),
        }
    }

    Ok(MusicSamples {
        all_samples: channels_data,
        sample_rate,
        channels
    })
}

pub fn save_music_samples(music_samples: &MusicSamples, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut interleaved = Vec::with_capacity(music_samples.all_samples[0].len() * music_samples.all_samples.len());
    for i in 0..music_samples.all_samples[0].len() {
        for ch in 0..music_samples.all_samples.len() {
            interleaved.push(music_samples.all_samples[ch][i]);
        }
    }

    let spec = hound::WavSpec {
        channels: music_samples.all_samples.len() as u16,
        sample_rate: music_samples.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("output.wav", spec)?;
    for s in interleaved {
        writer.write_sample((s * i16::MAX as f32) as i16)?;
    }
    writer.finalize()?;

    println!("Wrote transformed audio to {}", output_path);

    Ok(())
}