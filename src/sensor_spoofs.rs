use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub fn get_volt() -> f64 {
    let mut rng = thread_rng(); // Create a random number generator

    // Create a normal distribution with mean = 0 and standard deviation = 3
    let normal = Normal::new(0.0, 3.0).unwrap();

    // Get a random number from the distribution
    let w = normal.sample(&mut rng);
    14.4 + w
}
