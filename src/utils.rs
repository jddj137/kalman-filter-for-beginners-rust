pub fn ascending_float_range(start: f64, end: f64, step: f64) -> Vec<f64> {
    // Create an iterator that generates values from 'start' to 'end' with 'step' size
    let float_range: Vec<f64> = (0..)
        .map(|x| start + (x as f64) * step) // Generate a series with a given step
        .take_while(|&x| x <= end) // Stop when the generated value exceeds 'end'
        .collect(); // Collect into a Vec

    return float_range;
}

// TODO: This needs to take in a path buffer and handle printing errors
pub fn print_names_of_mat_file_arrays() {
    let file = std::fs::File::open("./data/SonarAlt_Ex02.mat")
        .expect("Failed to open: ./data/SonarAlt_Ex02.mat");
    let mat_file =
        matfile::MatFile::parse(file).expect("Failed to parse: ./data/SonarAlt_Ex02.mat");

    for array in mat_file.arrays() {
        println!(
            "Found array named {} of size {:?}",
            array.name(),
            array.size()
        );
    }
}

pub struct PlotLabels {
    pub plot_pathname: String,
    pub title: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
    pub y_axis_data1_label: String,
    pub y_axis_data2_label: String,
}
