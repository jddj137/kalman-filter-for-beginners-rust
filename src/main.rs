#![allow(non_snake_case)] // so units may be capitalized, e.g. Volts, V

pub mod recursive_filters;
pub mod sensor_spoofs;
pub mod utils;

use plotters::prelude::*;
use recursive_filters::AverageFilter;
use utils::ascending_float_range;

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

    // Configure the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Average Filter", ("sans-serif", 30).into_font())
        .margin(25)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..10f64, 4f64..24f64)
        .expect("ChartBuilder failed");

    // Configure mesh with axis labels and grid lines
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .y_labels(11)
        .x_desc("Time [s]") // Label for the x-axis
        .y_desc("Voltage [V]") // Label for the y-axis
        .x_label_style(("sans-serif", 18).into_font())
        .y_label_style(("sans-serif", 18).into_font())
        .x_label_formatter(&|x| format!("{}", *x as i64))
        .y_label_formatter(&|y| format!("{}", *y as i64))
        .draw()
        .expect("configure_mesh() failed");

    // Plot raw data with a red line
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

    // Plot filtered data with a blue line
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
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
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
