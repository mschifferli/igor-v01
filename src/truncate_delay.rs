use std::sync::{Arc, RwLock};

use super::effect::BufferedEffect;

const FADE_SAMPLE_COUNT: usize = 50;

// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
fn rand(n: f64) -> f64 {
  let s = (n * 43758.5453123).sin();
  let f = s.fract();
  // println!("rand s {} f {}", s, f ); 
  (n * 43758.5453123).sin().abs().fract()
}

pub struct TruncateDelay {
  samples_per_beat: usize,
  beats_per_repeat: usize,
  samples_per_bar: usize,
  seed: f64,
  index: usize,
  end_index: usize,
  len: usize,
  count: usize,
  fade_out: usize,
  buffer: Arc<RwLock<Vec<f32>>>
}



impl TruncateDelay {
    pub fn new(samples_per_beat: usize, 
          beats_per_repeat: usize, 
          buffer: Arc<RwLock<Vec<f32>>>,
          seed: f64) -> Self {
        let mut delay = TruncateDelay {
          samples_per_beat: samples_per_beat,
          beats_per_repeat: beats_per_repeat,
          samples_per_bar: samples_per_beat * beats_per_repeat,
          seed: seed,
          index: 0,
          end_index: 0,
          len: 1,
          count: 0,
          fade_out: samples_per_beat - FADE_SAMPLE_COUNT,
          buffer: buffer
        };
        delay
    }

    fn rand_int(&mut self, range: usize) -> usize {
      self.seed += 1.0;
      let r = rand(self.seed);
      // println!("rand_int {} {}", r, r * range as f64);
      (r * range as f64) as usize
      // (rand(self.seed) * range as f64) as usize
    }

    fn next_group(&mut self, index: usize) {
        self.len = self.rand_int(self.beats_per_repeat - 1) + 1;
        self.index = index - self.len * self.samples_per_beat;
        self.end_index = index;
        self.count = 0;
        self.fade_out = index - FADE_SAMPLE_COUNT;
        println!("start_bar {} {} {}", self.len, index, self.index);
    }
    
}

impl BufferedEffect for TruncateDelay {
    fn process_sample(&mut self, index: usize) -> f32 {
        if index < self.samples_per_bar {
            0.0
        } else {
            if self.index >= self.end_index {
                self.next_group(index);
            }
            // println!("   self.beat_index {:?}", self.beat_index);
            let mut anti_pop: f32 = 1.0;
            if self.count < FADE_SAMPLE_COUNT {
                anti_pop = self.count as f32 / FADE_SAMPLE_COUNT as f32; 
            } else if self.count > self.fade_out {
                anti_pop = (self.samples_per_beat - self.count) as f32 / FADE_SAMPLE_COUNT as f32; 
            }
            // println!("   anti_pop {:?}", anti_pop);
            let buffer = self.buffer.read().unwrap();
            // println!("   unwrapped");
            let sample = buffer[self.index as usize];
            self.count += 1;  
            self.index += 1;
            if self.count == self.samples_per_beat {
              self.count = 0;
            }
            // println!("{} {:?}", self.index, sample);
            sample * anti_pop
        }
    }
}

