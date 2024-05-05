pub mod recursive_filters;
pub mod sensor_spoofs;
pub mod utils;

use plotters::prelude::*;
use recursive_filters::{AverageFilter, MovingAverageFilter};
use utils::ascending_float_range;

fn test_average_filter() {
    // Setup simulation & data logging; inputs based on textbook example
    let times_s: Vec<f64> = ascending_float_range(0.0, 10.0, 0.2);

    let num_data_pts: usize = times_s.len();

    let mut raw_data = Vec::<f64>::with_capacity(num_data_pts);
    let mut filtered_data = Vec::<f64>::with_capacity(num_data_pts);

    // Initialize averaging filter
    let mut avg_filt = AverageFilter::new();

    // Run simulation
    for _ in 0..num_data_pts {
        let data_pt = sensor_spoofs::get_volt();
        avg_filt.update(data_pt);

        // Log data for plotting
        raw_data.push(data_pt);
        filtered_data.push(avg_filt.get_average());
    }

    // --- MAKE PLOTS --------------------------------------------------------//
    // Build and save graph using plotters crate; graph format based on textbook example
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
        .x_labels(10) // increments of 1
        .y_labels(11) // increments of 2
        .x_desc("Time [s]") // Label for the x-axis
        .y_desc("Voltage [V]") // Label for the y-axis
        .x_label_style(("sans-serif", 18).into_font())
        .y_label_style(("sans-serif", 18).into_font())
        .x_label_formatter(&|x| format!("{}", *x as i64))
        .y_label_formatter(&|y| format!("{}", *y as i64))
        .draw()
        .expect("configure_mesh() failed");

    // Plot the raw data as a red line, then add points
    chart
        .draw_series(LineSeries::new(
            times_s
                .iter()
                .zip(raw_data.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &RED,
        ))
        .expect("draw_series() LineSeries raw data failed");

    chart
        .draw_series(PointSeries::of_element(
            times_s.iter().zip(raw_data.iter()).map(|(&x, &y)| (x, y)),
            4, // Size of the points
            &RED,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Cross::new((0, 0), size, style.filled());
            },
        ))
        .expect("draw_series() PointSeries raw data failed")
        .label("Raw Data")
        .legend(|(x, y)| EmptyElement::at((x + 10, y)) + Cross::new((0, 0), 3, RED.filled()));

    // Plot the filtered data as a blue line, then add points
    chart
        .draw_series(LineSeries::new(
            times_s
                .iter()
                .zip(filtered_data.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &BLUE,
        ))
        .expect("draw_series() LineSeries filtered data failed");

    chart
        .draw_series(PointSeries::of_element(
            times_s
                .iter()
                .zip(filtered_data.iter())
                .map(|(&x, &y)| (x, y)),
            3, // Size of the points
            &BLUE,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled());
            },
        ))
        .expect("draw_series() PointSeries raw data failed")
        .label("Filtered Data")
        .legend(|(x, y)| EmptyElement::at((x + 10, y)) + Circle::new((0, 0), 3, BLUE.filled()));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .margin(5)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .expect("configure_series_labels() failed");

    let _ = root.present();

    println!("Average filter plot written: ./plots/AverageFilter.png");
}

fn test_moving_average_filter() {
    // Load sonar altitude simulation data
    let file = std::fs::File::open("./data/SonarAlt_Ex02.mat")
        .expect("Failed to open: ./data/SonarAlt_Ex02.mat");
    let mat_file =
        matfile::MatFile::parse(file).expect("Failed to parse: ./data/SonarAlt_Ex02.mat");

    if let Some(sonar_alt_arr) = mat_file.find_by_name("sonarAlt") {
        // Setup simulation & data logging; inputs based on textbook example
        let num_data_pts: usize = 500; // from example code

        let dt: f64 = 0.02; // from example code
        let times_s: Vec<f64> = ascending_float_range(0.0, dt * num_data_pts as f64, dt);

        let mut raw_data = Vec::<f64>::with_capacity(num_data_pts);
        let mut filtered_data = Vec::<f64>::with_capacity(num_data_pts);

        // Initialize moving averaging filter, window size = 10
        let mut mv_avg_filt = MovingAverageFilter::new(10);

        // Run simulation
        if let matfile::NumericData::Double { real, .. } = sonar_alt_arr.data() {
            for i in 0..num_data_pts {
                let data_pt = real[i];
                mv_avg_filt.update(data_pt);

                // Log data for plotting
                raw_data.push(data_pt);
                filtered_data.push(mv_avg_filt.get_average());
            }
        }

        // --- MAKE PLOTS ----------------------------------------------------//
        // Build and save graph using plotters crate; graph format based on textbook example
        let root =
            BitMapBackend::new("./plots/MovingAverageFilter.png", (640, 480)).into_drawing_area();
        let _ = root.fill(&WHITE);

        // Configure the chart
        let mut chart = ChartBuilder::on(&root)
            .caption("Moving Average Filter", ("sans-serif", 30).into_font())
            .margin(25)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..10f64, 30f64..120f64)
            .expect("ChartBuilder failed");

        // Configure mesh with axis labels and grid lines
        chart
            .configure_mesh()
            .x_labels(10) // increments of 1
            .y_labels(10) // increments of 10
            .x_desc("Time [s]") // Label for the x-axis
            .y_desc("Altitude [m]") // Label for the y-axis
            .x_label_style(("sans-serif", 18).into_font())
            .y_label_style(("sans-serif", 18).into_font())
            .x_label_formatter(&|x| format!("{}", *x as i64))
            .y_label_formatter(&|y| format!("{}", *y as i64))
            .draw()
            .expect("configure_mesh() failed");

        // Plot the raw data as red points
        chart
            .draw_series(PointSeries::of_element(
                times_s.iter().zip(raw_data.iter()).map(|(&x, &y)| (x, y)),
                2, // Size of the points
                &RED,
                &|coord, size, style| {
                    return EmptyElement::at(coord)
                        + Cross::new((0, 0), size, style.color.mix(0.8));
                },
            ))
            .expect("draw_series() PointSeries raw data failed")
            .label("Measured")
            .legend(|(x, y)| EmptyElement::at((x + 10, y)) + Cross::new((0, 0), 3, RED.filled()));

        // Plot the filtered data as a blue line
        chart
            .draw_series(LineSeries::new(
                times_s
                    .iter()
                    .zip(filtered_data.iter())
                    .map(|(&x_val, &y_val)| (x_val, y_val)),
                &BLUE,
            ))
            .expect("draw_series() LineSeries filtered data failed")
            .label("Filtered Data")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::LowerRight)
            .margin(5)
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .expect("configure_series_labels() failed");

        let _ = root.present();

        println!("Moving average filter plot written: ./plots/MovingAverageFilter.png");
    }
}

fn main() {
    test_average_filter();
    test_moving_average_filter();
}
