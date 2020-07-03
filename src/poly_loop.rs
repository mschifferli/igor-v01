use std::sync::{Arc, Mutex};

use super::effect::BufferedEffect;

const FADE_SAMPLE_COUNT: usize = 50;

pub struct PolyLoop {
  samples_per_beat: usize,
  beats_per_repeat_1: usize,
  beats_per_repeat_2: usize,
  samples_per_bar_1: usize,
  samples_per_bar_2: usize,
  index_1: usize,
  index_2: usize,
  fade_out_1: usize,
  fade_out_2: usize,
  buffer: Arc<Mutex<Vec<f32>>>
}



impl PolyLoop {
    pub fn new(samples_per_beat: usize, 
          beats_per_repeat_1: usize, 
          beats_per_repeat_2: usize, 
          buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        let mut delay = PolyLoop {
          samples_per_beat: samples_per_beat,
          beats_per_repeat_1: beats_per_repeat_1,
          beats_per_repeat_2: beats_per_repeat_2,
          samples_per_bar_1: samples_per_beat * beats_per_repeat_1,
          samples_per_bar_2: samples_per_beat * beats_per_repeat_2,
          index_1: 0,
          index_2: 0,
          fade_out_1: samples_per_beat * beats_per_repeat_1 - FADE_SAMPLE_COUNT,
          fade_out_2: samples_per_beat * beats_per_repeat_2 - FADE_SAMPLE_COUNT,
          buffer: buffer
        };
        delay
    }
    
}

impl BufferedEffect for PolyLoop {
    fn process_sample(&mut self, index: usize) -> f32 {
        let mut o:f32 = 0.0;
        let buffer = self.buffer.lock().unwrap();
        if index >= self.samples_per_bar_1 {
            // println!("   self.beat_index {:?}", self.beat_index);
            let mut anti_pop: f32 = 1.0;
            if self.index_1 < FADE_SAMPLE_COUNT {
                anti_pop = self.index_1 as f32 / FADE_SAMPLE_COUNT as f32; 
            } else if self.index_1 > self.fade_out_1 {
                anti_pop = (self.samples_per_bar_1 - self.index_1) as f32 / FADE_SAMPLE_COUNT as f32; 
            }
            self.index_1 += 1;
            if self.index_1 == self.samples_per_bar_1 {
              self.index_1 = 0;
            }
            // println!("{} {:?}", self.index, sample);
            o = buffer[self.index_1 as usize] * anti_pop;
        }
        if index >= self.samples_per_bar_2 {
            // println!("   self.beat_index {:?}", self.beat_index);
            let mut anti_pop: f32 = 1.0;
            if self.index_2 < FADE_SAMPLE_COUNT {
                anti_pop = self.index_2 as f32 / FADE_SAMPLE_COUNT as f32; 
            } else if self.index_2 > self.fade_out_2 {
                anti_pop = (self.samples_per_bar_2 - self.index_2) as f32 / FADE_SAMPLE_COUNT as f32; 
            }
            self.index_2 += 1;
            if self.index_2 == self.samples_per_bar_2 {
              self.index_2 = 0;
            }
            // println!("{} {:?}", self.index, sample);
            o += buffer[self.index_2 as usize] * anti_pop;
        }
        o
    }
}

