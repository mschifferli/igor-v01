use super::effect::Effect;

pub struct SymSoftClip {}


// https://books.google.com/books?id=h90HIV0uwVsC&pg=PA110&lpg=PA110&dq=symmetrical+soft+clipping&source=bl&ots=dlCbNApbi_&sig=ACfU3U0njUnz0grLeN-U0imAcklXhpCdVg&hl=en&sa=X&ved=2ahUKEwi7hJ75vKPqAhWEgnIEHcQBAi0Q6AEwC3oECAkQAQ#v=onepage&q=symmetrical%20soft%20clipping&f=false

impl SymSoftClip {
    pub fn new() -> Self {
        let dist = SymSoftClip {};
        dist
    }
}

impl Effect for SymSoftClip {
    fn process_sample(&mut self, input: f32) -> f32 {
        // input * input * (3.0 - 2.0 * input.abs()) * input.signum()
        let x = input.abs();
        6.0 * x.powi(5) - 15.0 * x.powi(4) + 10.0 * x.powi(3) * input.signum()
    }
}



