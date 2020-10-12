use std::sync::{Arc, RwLock};

// use super::effect::BufferedEffect;
use super::effect::Effect;

const FADE_SAMPLE_COUNT: usize = 50;

// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
fn rand(n: f64) -> f64 {
  let s = (n * 43758.5453123).sin();
  let f = s.fract();
  // println!("rand s {} f {}", s, f ); 
  (n * 43758.5453123).sin().abs().fract()
}

pub struct ScrambleDelay {
  samples_per_beat: usize,
  samples_per_bar: usize,
  seed: f64,
  index: usize,
  count: usize,
  // beat_start: usize, 
  bar_start: usize,
  beat_index: usize,
  order: Vec<usize>,
  fade_out: usize,
  buffer: Arc<RwLock<Vec<f32>>>,
  index: usize
}



impl ScrambleDelay {
    pub fn new(samples_per_beat: usize, 
          beats_per_repeat: usize, 
          buffer: Arc<RwLock<Vec<f32>>>,
          seed: f64) -> Self {
        let b = beats_per_repeat;
        let mut order: Vec<usize> = vec![0; b];
        for i in 0..b {
          order[i] = i;
        }

        let mut delay = ScrambleDelay {
          samples_per_beat: samples_per_beat,
          samples_per_bar: samples_per_beat * b,
          seed: seed,
          index: 0,
          count: 0,
          // beat_start: 0, 
          bar_start: 0,
          beat_index : 0, 
          order: order,
          fade_out: samples_per_beat - FADE_SAMPLE_COUNT,
          buffer: buffer,
          index : 0
        };
        delay.shuffle();
        // delay.set_beat();
        delay
    }

    fn swap(&mut self, i1: usize, i2: usize) {
      // println!("swap b4 {} {} {} {} {:?}", i1, self.order[i1], i2, self.order[i2], self.order);
      let t = self.order[i1];
      self.order[i1] = self.order[i2];
      self.order[i2] = t;
      // println!("swap after {} {} {} {} {:?}", i1, self.order[i1], i2, self.order[i2], self.order);
    }

    fn shuffle(&mut self) {
      println!("shuffle");
      for i in (1..self.order.len()).rev() {
          let i2 = self.rand_int(i);
          // println!("shuffle {} {}", i, i2);
          self.swap(i, i2);
      } 
      println!("{:?}", self.order); 
    }

    fn rand_int(&mut self, range: usize) -> usize {
      self.seed += 1.0;
      let r = rand(self.seed);
      // println!("rand_int {} {}", r, r * range as f64);
      (r * range as f64) as usize
      // (rand(self.seed) * range as f64) as usize
    }

    fn start_bar(&mut self, index: usize) {
        println!("start_bar {:?}", index);
        self.bar_start = index;
        self.shuffle();
        self.beat_index = 0; 
    }

    fn set_beat(&mut self) {
        let current_beat = self.order[self.beat_index];
        self.index = self.bar_start + current_beat * self.samples_per_beat;
        println!("set_beat {} {} {} {}", self.beat_index, current_beat, self.bar_start, self.index);
    }
    
}

impl Effect for ScrambleDelay {
    fn process_sample(&mut self, _input: f32) -> f32 {
        let index = self.index;
        self.index += 1;
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

