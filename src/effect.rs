pub trait Effect {
    fn process_sample(&mut self, input: f32) -> f32;
}

pub trait BufferedEffect {
    fn process_sample(&mut self, index: usize) -> f32;
}

pub trait Source {
    fn get_sample(&mut self) -> f32;
}

