//! Distort input like a fuzz box
//!
//! Audio from the default input device is passed through a filter and
//! then directly to the default output device in a duplex stream, so
//! beware of feedback!
extern crate portaudio;
extern crate nalgebra_glm as glm;


use three;
use portaudio as pa;
// use hound;

use std::sync::{Arc, RwLock};

mod effect;
// mod fuzz;
mod distortion;
mod lynch_delay;
mod echo;
// mod beat_delay;
// mod scramble_delay;
// mod truncate_delay;
// mod truncate_loop;
mod poly_loop;
// mod delay;
// mod beat_delay;
mod metronome;
use effect::{Effect, BufferedEffect, RingBufferEffect, Source};

#[derive(Debug)]
struct State {
    sound_values: Vec<f32>,
    scene_meshes: Vec<three::Mesh>
}


struct State2<'a> {
  input: &'a [f32],
  index: usize
}

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 64;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;

const PRE: f32 = 1.0;
// const GAIN: f32 = 0.25;

const COUNT_DOWN_BEATS : f64 = 8.0;

// const DRY_MIX: f32 = 0.0;
const TEMPO_MIX: f32 = 0.01;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
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

    let mut met = metronome::Metronome::new(140.0, TEMPO_MIX);

    let samples_per_beat: usize = met.samples_per_beat();
    let beats_per_repeat: usize = 10;
    let mut count_down = COUNT_DOWN_BEATS * met.beat_duration();
    let bars_to_record: usize = 128;
    let duration = (bars_to_record * beats_per_repeat) as f64 * met.beat_duration();
    let length : usize = (((SAMPLE_RATE  * duration ) as i32) * CHANNELS) as usize;
    



    let _att: f32 = 0.5;
    let buffer: Vec<f32> = vec![0.0; length];
    let buffer = Arc::new(RwLock::new(buffer));
    let callback_buffer = Arc::clone(&buffer);
    let mut index: usize = 0;
    let max_index = Arc::new(RwLock::new(0));
    let max_index = Arc::clone(&max_index);
    
    let recording: Vec<f32> = vec![0.0; length];
    let recording = Arc::new(RwLock::new(recording));
    let callback_recording = Arc::clone(&recording);

    let echo = echo::Echo::new(0.25);
    let echo = Arc::new(RwLock::new(echo));
    let callback_echo = Arc::clone(&echo);

    
    // let fuzz = symsoftclip::SymSoftClip::new();
    let fuzz = distortion::Distortion::new(2.0);
    // let fuzz = fuzz::Fuzz::new(8);
    let fuzz = Arc::new(RwLock::new(fuzz));
    let callback_fuzz = Arc::clone(&fuzz);

    // let delay = scramble_delay::ScrambleDelay::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer), 1.0);
    // let delay = delay::Delay::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer));
    // let delay = lynch_delay::LynchDelay::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer));
    // let delay = beat_delay::BeatDelay::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer));
    // let delay = truncate_delay::TruncateDelay::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer), 1.0);
    // let delay = truncate_loop::TruncateLoop::new(samples_per_beat, beats_per_repeat, Arc::clone(&buffer), 1.0);
    let delay = poly_loop::PolyLoop::new(samples_per_beat, beats_per_repeat, beats_per_repeat - 1, Arc::clone(&buffer));
    
    let delay = Arc::new(RwLock::new(delay));
    let callback_delay = Arc::clone(&delay);



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

        // assert!(frames == FRAMES as usize);
        let mut o : f32;
        let mut fuzz = callback_fuzz.write().unwrap();
        let mut delay = callback_delay.write().unwrap(); 
        let mut echo = callback_echo.write().unwrap();
        let mut rec = callback_recording.write().unwrap();


        if count_down > 0.0 {
          count_down -= dt;
          for (output_sample, _input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
              o = met.get_sample();
              *output_sample = o
          }
          sender.send( State2 {
              input: &[],
              index: 0
          }).ok();
          pa::Continue
        } else {
          // Pass the input through the fuzz filter and then to the output
          // BEWARE OF FEEDBACK!
          // duration -= dt;
          for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
              // o = 0.0; //*input_sample * PRE;
              o = *input_sample * PRE;
              {
                let mut buff = callback_buffer.write().unwrap();
                buff[index] = o;
              }
              
              // o = fuzz.process_sample(o);
              // let o2 = delay.process_sample(index);
              // rec[index] = o2;
              // o += o2;
              // o += echo.process_sample(o);
              o += delay.process_sample(index);
              rec[index] = o;
              o += met.get_sample();
              index += 1;
              if index >= length {
                index = 0;
                println!("Overwriting in buffer");
              }
              // *output_sample = o
              // *output_sample = o.clamp(-1.0, 1.0)
              *output_sample = o.max(-1.0).min(1.0)
          }
          // let end = index as usize;
          // println!("in_buffer: {:?} \n\t {:?}", duration, in_buffer[0]);
          // println!("in_buffer   : {:?}", in_buffer[0]);
          match sender.send(State2 {
              input: in_buffer,
              index: index
          }) {
              Ok(_) => portaudio::Continue, 
              Err(_) => portaudio::Complete
          }
          // } else {
          //     // println!("{:?}", buffer);
          //     pa::Complete
          // } 
        }
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    // Loop while the non-blocking stream is active.
    // while let true = stream.is_active()? {
    //     // Watch the count down while we wait for the stream to finish
    //     while let Ok(count_down) = receiver.try_recv() {
    //         // println!("count_down: {:?}", count_down);
    //     }
    // }

    let mut builder = three::Window::builder("Igor"); 
    builder.fullscreen(true); 
    let mut win = builder.build(); 
    win.scene.background = three::Background::Color(0x000000);
    let mut state = State {
        sound_values: Vec::new(),
        scene_meshes: Vec::new()
    };

    let camera = win.factory.orthographic_camera([0.0, 0.0], 1.0, -1.0 .. 1.0); 

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        update_lines(&mut win, &mut state);
        win.render(&camera);
        remove_lines(&mut win, &mut state);

        while let Ok(stream_state) = receiver.try_recv() {
            // println!("count_down: {:?} ", stream_state.duration);
            update_sound_values(stream_state.input, &mut state);   
            let mut mx = max_index.write().unwrap();
            *mx = stream_state.index;
       }

    }

    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input).ok();
    stream.stop()?;



    let index = max_index.read().unwrap();
    let index = *index;
    let wav_mix = recording.read().unwrap();
    let wav_mix = &wav_mix[0..index];
    // println!("final: {} {:?}", index, wav_raw);
    // normalize the output against the output max
    let mx = wav_mix.iter().fold(0.0, |a:f32, &b| a.max(b.abs()));
    println!("max {:?}", mx);
    let mult = 1.0 / mx;


    let wav_raw = buffer.read().unwrap();
    let wav_raw = &wav_raw[0..index];
    
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let raw_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/raw.wav");
    let mut raw_writer = hound::WavWriter::create(raw_path, spec).unwrap();
    for s in wav_raw.iter() {
        raw_writer.write_sample(s * mult).unwrap();
    }
    raw_writer.finalize().unwrap();

    let mix_path = concat!(env!("CARGO_MANIFEST_DIR"), "/mix.wav");
    let mut mix_writer = hound::WavWriter::create(mix_path, spec).unwrap();
    for s in wav_mix.iter() {
        mix_writer.write_sample(s * mult).unwrap();
    }
    mix_writer.finalize().unwrap();

    Ok(())
}



fn update_sound_values(samples: &[f32], state: &mut State) {
   state.sound_values = samples.to_vec(); 
}

fn update_lines(win: &mut three::window::Window, state: &mut State) {
    let num_samples = state.sound_values.len() as f32; 
    let scale = 3.0; 
    for (index, y_position) in state.sound_values.iter().enumerate() {
        let i = index as f32; 
        let x_position = (i / (num_samples / scale)) - (0.5 * scale);
        let material = three::material::Line {
            color: 0xFFFFFF,
        };
        let geometry = three::Geometry::with_vertices(vec![
            [x_position, y_position.clone(), 0.0].into(),
            [x_position, -y_position.clone(), 0.0].into()
        ]);
        let mesh = win.factory.mesh(geometry, material);
        win.scene.add(&mesh); 
        state.scene_meshes.push(mesh); 
    }
}

fn remove_lines(win: &mut three::window::Window, state: &mut State) {
    for mesh in &state.scene_meshes {
        win.scene.remove(&mesh); 
    }

    state.scene_meshes.clear(); 
}