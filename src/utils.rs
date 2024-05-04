pub fn ascending_float_range(start: f64, end: f64, step: f64) -> Vec<f64> {
    // Create an iterator that generates values from 'start' to 'end' with 'step' size
    let float_range: Vec<f64> = (0..)
        .map(|x| start + (x as f64) * step) // Generate a series with a given step
        .take_while(|&x| x <= end) // Stop when the generated value exceeds 'end'
        .collect(); // Collect into a Vec

    return float_range;
}
