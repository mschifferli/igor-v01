use super::effect::Effect;

pub struct Distortion { amt: f32}


// const E: f64 = 2.71828;

impl Distortion {
    pub fn new(f:f32) -> Self {
        let dist = Distortion {amt: f};
        dist
    }
}

impl Effect for Distortion {
    fn process_sample(&mut self, input: f32) -> f32 {
        (1.0 - (1.0 - input.abs()).powf(self.amt)) * input.signum() 
    }
}



