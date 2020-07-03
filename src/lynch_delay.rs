use std::sync::{Arc, Mutex};

use super::effect::BufferedEffect;

const FADE_SAMPLE_COUNT: i32 = 50;

pub struct LynchDelay {
  samples_per_beat: i32,
  jump: i32,
  back: i32,
  // FADE_SAMPLE_COUNT * 2.0 because when playing reverse samples 
  // we increment the counter by 2
  back_fade_in: i32,
  back_fade_out: i32,
  buffer: Arc<Mutex<Vec<f32>>>
}



impl LynchDelay {
    pub fn new(samples_per_beat: usize, beats_per_repeat: usize, buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        let samps = samples_per_beat as i32;
        let delay = LynchDelay {
          samples_per_beat: samps,
          jump: samps * beats_per_repeat as i32,
          back: samps - 1,
          back_fade_in: samps - FADE_SAMPLE_COUNT * 2,
          back_fade_out: -samps + FADE_SAMPLE_COUNT * 2,
          buffer: buffer
        };
        delay
    }
}

impl BufferedEffect for LynchDelay {
    fn process_sample(&mut self, index: usize) -> f32 {
        let i2 = (index as i32) - self.jump + self.back;
        if i2 >= 0 {
            // attenuation = 0.9;
            let mut anti_pop: f32 = 1.0;
            if self.back <= -self.samples_per_beat {
                // let b4 = back;
                self.back = self.samples_per_beat - 1;
                // println!("b4 {:?} back {:?} samples_per_beat {:?}", b4, back, samples_per_beat);
                anti_pop = 0.0;
                // println!("at {:?}", anti_pop);
            } else if self.back > self.back_fade_in {
                anti_pop = (self.samples_per_beat - self.back) as f32 / FADE_SAMPLE_COUNT as f32; 
                // println!("fade in {:?}", anti_pop);
            } else if self.back < self.back_fade_out {
                anti_pop = (self.back + self.samples_per_beat) as f32 / FADE_SAMPLE_COUNT as f32; 
                // println!("fade out {:?}", anti_pop);
            } 
            // while i2 >= 0 && attenuation > 0.05 {
            //   o += buff[i2 as usize] * attenuation;
            //   i2 -= jump;
            //   // attenuation *= att;
            //   attenuation = 0.0;
            // }
            self.back -= 2;         
            let buffer = self.buffer.lock().unwrap();
            buffer[i2 as usize] * anti_pop
        } else {
            0.0          
        }
    }
}

