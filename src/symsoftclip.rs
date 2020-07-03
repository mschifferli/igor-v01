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
        let x = input.abs();
        if x < 0.33 {
            input * 2.0
        } else if x < 0.67 {
            (3.0 - (2.0 - 3.0 * input).powi(2)) / 3.0 * input.signum()
        } else {
            input
        }

        // or 
        // (input - input.powi(3) / 3.0) * 1.333333 
    }
}



