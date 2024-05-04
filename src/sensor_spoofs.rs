use rand::random;

pub fn get_volt() -> f64 {
    let w = 0.0 + 4.0 * random::<f64>();
    14.4 + w
}
