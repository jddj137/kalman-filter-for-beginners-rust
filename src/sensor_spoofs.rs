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

static TRUE_POS_A: Mutex<f64> = Mutex::new(0.0);
static TRUE_POS_B: Mutex<f64> = Mutex::new(0.0);

pub static TRUE_VEL_A: Mutex<f64> = Mutex::new(80.0);
static TRUE_VEL_B: Mutex<f64> = Mutex::new(80.0);

const DT: f64 = 0.1;

pub fn get_position() -> f64 {
    // Create a random number generator
    let mut rng = thread_rng();
    // Create a normal distribution with mean = 0 and standard deviation = 1
    let normal = Normal::new(0.0, 1.0).unwrap();

    let w: f64 = 0.0 + 10.0 * normal.sample(&mut rng);
    let v: f64 = 0.0 + 10.0 * normal.sample(&mut rng);

    let mut true_pos = TRUE_POS_A.lock().unwrap();
    let mut true_vel = TRUE_VEL_A.lock().unwrap();

    let z: f64 = *true_pos + *true_vel * DT + v;

    *true_pos = z - v; // true position
    *true_vel = 80.0 + w; // true speed

    return z;
}

pub fn get_velocity() -> f64 {
    // Create a random number generator
    let mut rng = thread_rng();
    // Create a normal distribution with mean = 0 and standard deviation = 1
    let normal = Normal::new(0.0, 1.0).unwrap();

    let v: f64 = 0.0 + 10.0 * normal.sample(&mut rng);

    let mut true_pos = TRUE_POS_B.lock().unwrap();
    let mut true_vel = TRUE_VEL_B.lock().unwrap();

    let z: f64 = 80.0 + v;

    *true_pos = *true_pos + *true_vel * DT; // true position
    *true_vel = z; // true speed

    return z;
}
