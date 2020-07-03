pub trait Effect {
    fn process_sample(&mut self, input: f32) -> f32;
}

pub trait BufferedEffect {
    fn process_sample(&mut self, index: usize) -> f32;
}



// pub struct Fuzz {}

// impl Fuzz {
//     pub fn new() -> Self {
//         let fuzz = Fuzz {};
//         fuzz
//     }
// }

// impl Effect for Fuzz {
//     fn process_sample(&mut self, input: f32) -> f32 {
//         (0..4).fold(input, |acc, _| cubic_amplifier(acc))
//     }
// }



// fn cubic_amplifier(input: f32) -> f32 {
//     // samples should be between -1.0 and 1.0
//     if input < 0.0 {
//         // if it's negative (-1.0 to 0), then adding 1.0 takes it to
//         // the 0 to 1.0 range. If it's cubed, it still won't leave the
//         // 0 to 1.0 range.
//         let negated = input + 1.0;
//         // (((negated * negated * negated) - 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
//         (negated * negated * negated) - 1.0
//     } else {
//         // if it's positive (0 to 1.0), then subtracting 1.0 takes it
//         // to the -1.0 to 0 range. If it's cubed, it still won't leave
//         // the -1.0 to 0 range.
//         let negated = input - 1.0;
//         // (((negated * negated * negated) + 1.0) * (1.0 - DRY_MIX) + input * DRY_MIX)
//         (negated * negated * negated) + 1.0

//     }
// }
//     // e = 2.71828
//     // if (input > 0)
//     //  y = 1 - exp(-x);
//     // else
//     //  y = -1 + exp(x);
//     // end


