//! Distort input like a fuzz box
//!
//! Audio from the default input device is passed through a filter and
//! then directly to the default output device in a duplex stream, so
//! beware of feedback!
extern crate portaudio;
extern crate nalgebra_glm as glm;


use portaudio as pa;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 64;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;

const DRY_MIX: f32 = 0.0;



fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

// the fuzz filter, when applied to all samples, will add some
// distortion
fn fuzz(input: f32) -> f32 {
    (0..4).fold(input, |acc, _| cubic_amplifier(acc))
}

fn cubic_amplifier(input: f32) -> f32 {
    // samples should be between -1.0 and 1.0
    if input < 0.0 {
        // if it's negative (-1.0 to 0), then adding 1.0 takes it to
        // the 0 to 1.0 range. If it's cubed, it still won't leave the
        // 0 to 1.0 range.
        let negated = input + 1.0;
        // (((negated * negated * negated) - 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
        (negated * negated * negated) - 1.0
    } else {
        // if it's positive (0 to 1.0), then subtracting 1.0 takes it
        // to the -1.0 to 0 range. If it's cubed, it still won't leave
        // the -1.0 to 0 range.
        let negated = input - 1.0;
        // (((negated * negated * negated) + 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
        (negated * negated * negated) + 1.0

    }
}

fn tick(metronome: i32) -> f32 {
  if metronome < 1000 {
    let mut s = 0.5;
    if metronome < 10 {
      s = metronome as f32 / 10.0;
    } else if metronome > 900 {
      s = 1.0 - (metronome as f32 - 900.0) / 100.0;
    }
    (metronome as f32 * 0.05).sin() * s
  } else {
    0.0
  }

}


fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;

    println!("PortAudio:");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", pa.host_api_count()?);

    let default_host = pa.default_host_api()?;
    println!("default host: {:#?}", pa.host_api_info(default_host));

    let def_input = pa.default_input_device()?;
    let input_info = pa.device_info(def_input)?;
    println!("Default input device info: {:#?}", &input_info);

    // Construct the input stream parameters.
    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);

    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    println!("Default output device info: {:#?}", &output_info);

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

    // Check that the stream format is supported.
    pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE)?;

    // Construct the settings with which we'll open our duplex stream.
    let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);



    // Keep track of the last `current_time` so we can calculate the delta time.
    let mut maybe_last_time = None;

    // We'll use this channel to send the count_down to the main thread for fun.
    let (sender, receiver) = ::std::sync::mpsc::channel();

    // let delay_length: f64 = 0.5;
    // let delay_line: DelayLine = delay_line::DelayLine::new((delay_length * SAMPLE_RATE) as usize );

    let delay = 8.0;
    let tempo = 120.0; // beats per minute
    let beat = 60.0 / tempo; // how many seconds does a beat last?
    let samplesPerBeat = (((SAMPLE_RATE * beat ) as i32) * CHANNELS) as i32;
    let beatsPerRepeat = 4.0;
    let mut count_down = (beatsPerRepeat + 1.0) * beat;
    let mut metronome = 0;
    let barsToRecord = 16.0;
    let mut duration = barsToRecord * beatsPerRepeat * beat;
    let length : usize = (((SAMPLE_RATE  * duration ) as i32) * CHANNELS) as usize;
    let mut buffer: Vec<f32> = vec![0.0; length];
    let mut index: usize = 0;



    let JUMP = (SAMPLE_RATE * delay ) as i32;
    // A callback to pass to the non-blocking stream.
    let callback = move |pa::DuplexStreamCallbackArgs {
                             in_buffer,
                             out_buffer,
                             frames,
                             time,
                             ..
                         }| {
        let current_time = time.current;
        let prev_time = maybe_last_time.unwrap_or(current_time);
        let dt = current_time - prev_time;
        maybe_last_time = Some(current_time);

        assert!(frames == FRAMES as usize);
        let mut o : f32 = 0.0;

        if count_down > 0.0 {
          count_down -= dt;
          for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
              let mut o = tick(metronome);
              metronome += 1;
              if metronome == samplesPerBeat {
                metronome = 0;
              }
              *output_sample = o
          }          
          sender.send(count_down).ok();
          pa::Continue
        } else {
          // Pass the input through the fuzz filter and then to the output
          // BEWARE OF FEEDBACK!
          duration -= dt;
          
          let mut i2 : i32;
          let ATT: f32 = 0.5;
          let mut attenuation: f32 = ATT;
          for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
              let mut o = tick(metronome);
              metronome += 1;
              if metronome == samplesPerBeat {
                metronome = 0;
              }
              o += fuzz(*input_sample);
              i2 = (index as i32) - JUMP;
              attenuation = 0.9;
              buffer[index] = o;
              index = index + 1;
              if index >= length {
                index = 0;
              }
              while i2 >= 0 && attenuation > 0.05 {
                o += buffer[i2 as usize] * attenuation;
                i2 -= JUMP;
                attenuation *= ATT;
              }
              *output_sample = o
          }

          sender.send(o.into()).ok();
          if duration > 0.0 {
              pa::Continue
          } else {
              println!("{:?}", buffer);
              pa::Complete
          } 
        }
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    // Loop while the non-blocking stream is active.
    while let true = stream.is_active()? {
        // Watch the countdown while we wait for the stream to finish
        while let Ok(count_down) = receiver.try_recv() {
            println!("count_down: {:?}", count_down);
        }
    }

    stream.stop()?;

    Ok(())
}
