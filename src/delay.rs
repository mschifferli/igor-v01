use std::sync::{Arc, Mutex};

use super::effect::BufferedEffect;

const FADE_SAMPLE_COUNT: i32 = 50;


fn rand(n: f64) -> f64 {
  let s = (n * 43758.5453123).sin();
  let f = s.fract();
  // println!("rand s {} f {}", s, f ); 
  (n * 43758.5453123).sin().abs().fract()
}

// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
pub struct Delay {
  samples_per_beat: i32,
  jump: i32,
  // FADE_SAMPLE_COUNT * 2.0 because when playing reverse samples 
  // we increment the counter by 2
  back_fade_in: i32,
  back_fade_out: i32,
  buffer: Arc<Mutex<Vec<f32>>>
}



impl Delay {
    pub fn new(samples_per_beat: usize, beats_per_repeat: usize, buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        let samps = samples_per_beat as i32;
        let delay = Delay {
          samples_per_beat: samps,
          jump: samps * beats_per_repeat as i32,
          back_fade_in: samps - FADE_SAMPLE_COUNT * 2,
          back_fade_out: -samps + FADE_SAMPLE_COUNT * 2,
          buffer: buffer
        };
        delay
    }
}

impl BufferedEffect for Delay {
    fn process_sample(&mut self, index: usize) -> f32 {
        let i2 = (index as i32) - self.jump;
        if i2 >= 0 {
            // attenuation = 0.9;
            let mut anti_pop: f32 = 1.0;            
            let buffer = self.buffer.lock().unwrap();
            buffer[i2 as usize] * anti_pop
        } else {
            0.0          
        }
    }
}

