use cpal::EventLoop;
use cpal::{StreamData, UnknownTypeOutputBuffer};

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("No output device available");
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported formats?!")
        .with_max_sample_rate();

    println!("{:?}", &format);

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let sample_rate = 1.0 / format.sample_rate.0 as f32;
    let freq = 440.0; //Hz
    let increment = 2.0 * std::f32::consts::PI * freq * sample_rate;
    let volume = 0.5;

    let mut counter: f32 = 0.0;
    let mut is_right = false;

    // This will lock up your thread, so should be moved to a different thread
    event_loop.run(move |_stream_id, stream_data| match stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                let target = counter.sin() * volume;

                let half = (u16::max_value() / 2) as f32;
                let val = (half + half * target) as u16;

                if is_right {
                    *elem = val;
                    counter += increment;
                } else {
                    *elem = half as u16;
                }
                is_right = !is_right;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                let target = counter.sin() * volume;

                let val = (target * i16::max_value() as f32) as i16;

                if is_right {
                    *elem = val;
                    counter += increment;
                } else {
                    *elem = 0;
                }
                is_right = !is_right;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                let target = counter.sin() * volume;

                if is_right {
                    *elem = target;
                    counter += increment;
                } else {
                    *elem = 0.0;
                }
                is_right = !is_right;
            }
        }
        _ => (),
    });
}
