use cpal::{
    traits::{EventLoopTrait, HostTrait},
    Format, SampleFormat, SampleRate, StreamData, UnknownTypeOutputBuffer,
};
use bytemuck::{try_from_bytes, cast_slice};

use earwax::Earwax;
use earwax::LogLevel;

const AUDIO_FILE: &str = "./tracks/Canon.mp3";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Earwax::set_log_level(LogLevel::Debug);

    let mut earwax = Earwax::new(AUDIO_FILE).unwrap();

    let mut audio = vec![];
    while let Some(chunk) = earwax.spit() {
        for sample_bytes in chunk.data.chunks_exact(4) {
            let sample = try_from_bytes(cast_slice(sample_bytes)).unwrap();
            audio.push(*sample);
        }
    }

    println!("{}", audio.len());

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let format = Format {
        channels: 2,
        data_type: SampleFormat::F32,
        sample_rate: SampleRate(earwax.info().sample_rate as u32),
    };

    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;
    event_loop.play_stream(stream_id)?;

    let mut audio_iter = audio.iter();

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };
        if let StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } = stream_data
        {
            for elem in buffer.iter_mut() {
                *elem = match audio_iter.next() {
                    Some(sample) => *sample,
                    _ => 0.0,
                };
            }
        }
    });
}
