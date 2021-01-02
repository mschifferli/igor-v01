extern crate ringbuf;

use ringbuf::{RingBuffer, Producer, Consumer};
// use super::effect::BufferedEffect;
use super::effect::Effect;

// straight up fuzz. 
// currently reps indicates a quantized amount
// of fuzzivess. Would be nice to do something 
// more gradual
pub struct Echo { 
  producer: Producer<f32>, 
  consumer: Consumer<f32>
}

impl Echo {
    pub fn new(latency: f32, sample_rate: f64, channels: i32) -> Self {
        // Create a delay in case the input and output devices aren't synced.
        let latency_frames = (latency / 1_000.0) * sample_rate as f32;
        let latency_samples = latency_frames as usize * channels as usize;
        let ring = RingBuffer::new(latency_samples * 2);
        // let (mut producer, mut consumer) = ring.split();
        let (mut producer, consumer) = ring.split();
        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }
        let echo = Echo { 
            // latency: latency, 
            producer: producer, 
            consumer: consumer
            // sample_rate: sample_rate,
            // channels: channels
        };
        echo
    }
}

impl Effect for Echo {
    fn process_sample(&mut self, input: f32) -> f32 {
        self.producer.push(input).unwrap();
        let sample = self.consumer.pop().unwrap();
        sample
    }
}




