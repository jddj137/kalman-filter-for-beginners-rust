use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use std::sync::Mutex;

pub fn get_volt() -> f64 {
    let mut rng = thread_rng(); // Create a random number generator

    // Create a normal distribution with mean = 0 and standard deviation = 4
    let normal = Normal::new(0.0, 4.0).unwrap();

    // Get a random number from the distribution
    let w = normal.sample(&mut rng);
    14.4 + w
}

static PRD_POS: Mutex<f64> = Mutex::new(0.0);
static PRD_VEL: Mutex<f64> = Mutex::new(80.0);
const DT: f64 = 0.1;

pub fn get_position() -> f64 {
    // Create a random number generator
    let mut rng = thread_rng();
    // Create a normal distribution with mean = 0 and standard deviation = 1
    let normal = Normal::new(0.0, 1.0).unwrap();

    let w: f64 = 0.0 + 10.0 * normal.sample(&mut rng);
    let v: f64 = 0.0 + 10.0 * normal.sample(&mut rng);

    let mut prd_pos = PRD_POS.lock().unwrap();
    let mut prd_vel = PRD_VEL.lock().unwrap();

    let z: f64 = *prd_pos + *prd_vel * DT + v;

    *prd_pos = z - v; // true position
    *prd_vel = 80.0 + w; // true speed

    return z;
}
