use std::sync::{Arc, RwLock};

use super::effect::Effect;

const FADE_SAMPLE_COUNT: usize = 50;


// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
pub struct BeatDelay {
  samples_per_beat: usize,
  samples_per_bar: usize,
  index: usize,
  count: usize,
  // beat_start: usize, 
  bar_start: usize,
  beat_index: usize,
  order: Vec<usize>,
  fade_out: usize,
  buffer: Arc<RwLock<Vec<f32>>>,
  playback_index: usize
}



impl BeatDelay {
    pub fn new(samples_per_beat: usize, 
          beats_per_repeat: usize, 
          buffer: Arc<RwLock<Vec<f32>>>) -> Self {
        let b = beats_per_repeat;
        let mut order: Vec<usize> = vec![0; b];
        for i in 0..b {
          order[i] = i;
        }

        let mut delay = BeatDelay {
          samples_per_beat: samples_per_beat,
          samples_per_bar: samples_per_beat * b,
          index: 0,
          count: 0,
          bar_start: 0,
          beat_index : 0, 
          order: order,
          fade_out: samples_per_beat - FADE_SAMPLE_COUNT,
          buffer: buffer,
          playback_index: 0
        };
        // delay.shuffle();
        delay.set_beat();
        delay
    }

    fn start_bar(&mut self, index: usize) {
        println!("start_bar {:?}", index);
        self.bar_start = index;
        // self.shuffle();
        self.beat_index = 0; 
    }

    fn set_beat(&mut self) {
        let current_beat = self.order[self.beat_index];
        self.index = self.bar_start + current_beat * self.samples_per_beat;
        println!("set_beat {} {} {} {}", self.beat_index, current_beat, self.bar_start, self.index);
    }
}

impl Effect for BeatDelay {
    fn process_sample(&mut self, _input: f32) -> f32 {
        let index = self.playback_index;
        self.playback_index += 1;
        if index < self.samples_per_bar {
            0.0
        } else {
            if self.count == 0 {
                if self.beat_index == self.order.len() {
                    self.start_bar(index - self.samples_per_bar);
                    // println!("at bar {} {} ", self.beat_index, index);
                }
                self.set_beat();
                self.beat_index += 1;              
            }
            // let mut anti_pop: f32 = 1.0;
            // if self.count < FADE_SAMPLE_COUNT {
            //     anti_pop = self.count as f32 / FADE_SAMPLE_COUNT as f32; 
            // } else if self.count > self.fade_out {
            //     anti_pop = (self.samples_per_beat - self.count) as f32 / FADE_SAMPLE_COUNT as f32; 
            // }
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
            // sample * anti_pop
            sample         
        }
    }
}

