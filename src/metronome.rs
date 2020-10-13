use super::effect::Source;


// const SAMPLE_RATE: f64 = 44_100.0;
// const CHANNELS: i32 = 2;

const METRO_PITCH: f32 = 440.0;
// how many samples in one cycle of the sine wave at this pitch?
// const SAMPLES_PER_CYCLE: f32 = SAMPLE_RATE / METRO_PITCH;
// how much does each sample need to progress to get this pitch?
// const METRO_INCREMENT : f32 = METRO_PITCH * (std::f64::consts::PI * 2.0 / SAMPLE_RATE) as f32 ;




pub struct Metronome { 
    samples_per_beat: usize,
    tempo: f32,
    gain: f32,
    tick: usize,
    sample_rate: f64,
    channels: i32,
    increment: f32
}

impl Metronome {
    pub fn new(
      tempo: f32,
      gain: f32,
      sample_rate: f64,
      channels: i32
      ) -> Self {
        let increment = METRO_PITCH * (std::f64::consts::PI * 2.0 / sample_rate) as f32 ;
        let mut metronome = Metronome { 
          samples_per_beat: 0,
          tempo: tempo,
          gain: gain,
          tick: 0,
          sample_rate: sample_rate,
          channels: channels,
          increment: increment
        };
        metronome.set_tempo(tempo);
        metronome
    }

    pub fn beat_duration(&mut self) -> f64 {
      60.0 / self.tempo as f64
    }

    pub fn samples_per_beat(&mut self) -> usize {
      self.samples_per_beat
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo;
        let beat: f64 = 60.0 / tempo as f64; // how many seconds does a beat last?
        self.samples_per_beat = ((self.sample_rate * beat ) as usize) * self.channels as usize;        
        if self.tick > self.samples_per_beat {
          self.tick = 0;
        }
    }
}

impl Source for Metronome {
    fn get_sample(&mut self) -> f32 {
        let mut o = 0.0;
        if self.tick < 512 {
          let mut s = self.gain;
          if self.tick < 10 {
            s *= self.tick as f32 / 10.0;
          } else if self.tick > 448 {
            s *= 1.0 - (self.tick as f32 - 448.0) / 64.0;
          }
          o = (self.tick as f32 * self.increment).sin() * s;
        }
        self.tick += 1;
        if self.tick == self.samples_per_beat {
          self.tick = 0;
          // println!("tick");
        }
        o
    }
}


