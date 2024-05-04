#![allow(non_snake_case)] // so units may be capitalized, e.g. Volts, V

pub mod recursive_filters;
pub mod sensor_spoofs;
pub mod utils;

use plotters::prelude::*;
use recursive_filters::AverageFilter;
use utils::generate_float_range;

fn test_average_filter() {
    // Setup simulation & data logging
    let times_s: Vec<f64> = ascending_float_range(0.0, 10.0, 0.2);

    let num_data_pts: usize = times_s.len();

    let mut raw_data_V = Vec::<f64>::with_capacity(num_data_pts);
    let mut filtered_data_V = Vec::<f64>::with_capacity(num_data_pts);

    // Initialize averaging filter
    let mut avg_filt = AverageFilter::new();

    // Run simulation
    for _ in 0..num_data_pts {
        let x = sensor_spoofs::get_volt();
        avg_filt.update(x);

        raw_data_V.push(x);
        filtered_data_V.push(avg_filt.get_average());
    }

    // plotters graphing configuration
    let root = BitMapBackend::new("./plots/AverageFilter.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Average Filter", ("sans-serif", 35).into_font())
        .margin(25)
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f64..10f64, 0f64..20f64)
        .expect("ChartBuilder failed");

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.2}", x))
        .draw()
        .expect("configure_mesh() failed");

    // Plot the raw data as a line
    chart
        .draw_series(LineSeries::new(
            times_s
                .iter()
                .zip(raw_data_V.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &RED,
        ))
        .expect("draw_series() raw data failed")
        .label("Raw Data")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Plot the filtered data as a line
    chart
        .draw_series(LineSeries::new(
            times_s
                .iter()
                .zip(filtered_data_V.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &BLUE,
        ))
        .expect("draw_series() filtered data failed")
        .label("Filtered Data")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .expect("configure_series_labels() failed");

    let _ = root.present();

    println!("Average Filter test plot written: ./plots/AverageFilter.png");
}

fn main() {
    test_average_filter();
}
