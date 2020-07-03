use super::effect::Effect;

const MAX_FUZZ: i32 = 2;


// used by the fuzz effects
fn cubic_amplifier(input: f32) -> f32 {
    // samples should be between -1.0 and 1.0
    if input < 0.0 {
        // if it's negative (-1.0 to 0), then adding 1.0 takes it to
        // the 0 to 1.0 range. If it's cubed, it still won't leave the
        // 0 to 1.0 range.
        let negated = input + 1.0;
        // (((negated * negated * negated) - 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
        (negated * negated * negated) - 1.0
    } else {
        // if it's positive (0 to 1.0), then subtracting 1.0 takes it
        // to the -1.0 to 0 range. If it's cubed, it still won't leave
        // the -1.0 to 0 range.
        let negated = input - 1.0;
        // (((negated * negated * negated) + 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
        (negated * negated * negated) + 1.0

    }
}




// straight up fuzz. 
// currently reps indicates a quantized amount
// of fuzzivess. Would be nice to do something 
// more gradual
pub struct Fuzz { reps: i32 }

impl Fuzz {
    pub fn new(r: i32) -> Self {
        let mut r2 = r;
        if r2 < 0 {
          r2 = 0;
        } else if r2 > MAX_FUZZ {
          r2 = MAX_FUZZ;
        }
        let fuzz = Fuzz { reps: r2 };
        fuzz
    }
}

impl Effect for Fuzz {
    fn process_sample(&mut self, input: f32) -> f32 {
        if input.abs() > 0.005 {
            (0..self.reps).fold(input, |acc, _| cubic_amplifier(acc))
        } else {
          0.0
        }
    }
}





// loops through the different fuzz steps
pub struct Fuzzicle {
  reps: i32,
  i: i32,
  d: i32
}

impl Fuzzicle {
    pub fn new(r: i32) -> Self {
        let mut r2 = r;
        if r2 < 0 {
          r2 = 0;
        } else if r2 > MAX_FUZZ {
          r2 = MAX_FUZZ;
        }
        let fuzz = Fuzzicle { reps: r2, i: 0, d: 1 };
        fuzz
    }
}

impl Effect for Fuzzicle {
    fn process_sample(&mut self, input: f32) -> f32 {
        if self.i == 100_000 {
            self.reps += self.d;
            if self.reps == MAX_FUZZ {
              self.d = -1;
            } else if self.reps == 0 {
              self.d = 1;
            }
            self.i = 0;
            println!("distortion intensity: {:?}", self.reps);
        }
        self.i += 1;
        if input.abs() > 0.005 {
            (0..self.reps).fold(input, |acc, _| cubic_amplifier(acc))
        } else {
          0.0
        }
    }
}





