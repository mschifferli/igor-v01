extern crate ringbuf;

use ringbuf::{RingBuffer, Producer, Consumer};
use super::effect::RingBufferEffect;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 64;
const CHANNELS: i32 = 2;
const LOWEST_PITCH: f32 = 30.0; // low B
const LATENCY_SAMPLES: usize = (SAMPLE_RATE / LOWEST_PITCH * CHANNELS) as usize;


pub struct Echo { 
  producer: Producer<f32>, 
  consumer: Consumer<f32>
}

impl Echo {
    pub fn new(latency: f32) -> Self {
        // Create a delay in case the input and output devices aren't synced.
        let ring = RingBuffer::new(LATENCY_SAMPLES);
        let (mut producer, mut consumer) = ring.split();
        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..LATENCY_SAMPLES {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }
        let echo = Echo { 
            latency: latency, 
            producer: producer, 
            consumer: consumer };
        echo
    }
}

impl RingBufferEffect for Echo {
    fn process_sample(&mut self, input: f32) -> f32 {
        self.producer.push(input);
        let sample = self.consumer.pop().unwrap();
        sample
    }
}




