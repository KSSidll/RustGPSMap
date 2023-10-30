use rand::{Rng, thread_rng};

#[allow(dead_code)]
pub fn generate_random_f32(from: f32, until: f32, accuracy: f32) -> f32 {
    (thread_rng().gen_range(from..=until) * (1.0 / accuracy)).round() / (1.0 / accuracy)
}

#[allow(dead_code)]
pub fn generate_random_i32(from: i32, until: i32) -> i32 {
    thread_rng().gen_range(from..=until)
}

