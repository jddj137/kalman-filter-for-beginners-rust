use crate::sensor_spoofs;
use crate::utils::{ascending_float_range, PlotLabels};
use kalman_filter_for_beginners_rust::kalman_filter::{KalmanFilter, SystemModel};
use nalgebra::{DMatrix, DVector};
use plotters::prelude::*;

pub fn kalman_filter_extremely_simple_example() {
    // Setup simulation & data logging; inputs based on textbook example
    let times_s: Vec<f64> = ascending_float_range(0.0, 10.0, 0.2);

    let num_data_pts: usize = times_s.len();

    let mut measurements_z = Vec::<f64>::with_capacity(num_data_pts);
    let mut estimates_x = Vec::<f64>::with_capacity(num_data_pts);
    let mut error_covariance_p = Vec::<f64>::with_capacity(num_data_pts);
    let mut kalman_gain_k = Vec::<f64>::with_capacity(num_data_pts);

    // Initialize system model
    let system_model = SystemModel::new(
        DMatrix::identity(1, 1),
        DMatrix::zeros(1, 1),
        DMatrix::identity(1, 1),
        DMatrix::from_element(1, 1, 4.0),
    );

    // Initialize Kalman filter
    let mut klmn_filt = KalmanFilter::new(
        system_model,
        DVector::from_element(1, 14.0),
        DMatrix::from_element(1, 1, 6.0),
    );

    // Log initial Kalman gain b/c it's calculated on initialization, before update
    kalman_gain_k.push(klmn_filt.get_kalman_gain()[0]);

    // Run simulation
    for _ in 0..num_data_pts {
        let data_pt = sensor_spoofs::get_volt();
        klmn_filt.update(DVector::from_element(1, data_pt));

        // Log data for plotting
        measurements_z.push(data_pt);
        estimates_x.push(klmn_filt.get_state_estimate()[0]);
        error_covariance_p.push(klmn_filt.get_error_covariance()[0]);
        kalman_gain_k.push(klmn_filt.get_kalman_gain()[0]);
    }

    // --- MAKE PLOTS --------------------------------------------------------//
    // Build and save graph using plotters crate; graph format based on textbook example
    let x_axis_data1 = measurements_z;
    let x_axis_data2 = estimates_x;
    let y_axis_data = times_s;

    let plot_labels = PlotLabels {
        plot_pathname: "./plots/04a_KalmanFilter_SimpleEx.png".to_string(),
        title: "Kalman Filter".to_string(),
        x_axis_label: "Time [s]".to_string(),
        y_axis_label: "Voltage [V]".to_string(),
        x_axis_data1_label: "Measurements".to_string(),
        x_axis_data2_label: "Kalman Filter".to_string(),
    };

    let root = BitMapBackend::new(&plot_labels.plot_pathname, (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    // Configure the chart
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_labels.title, ("sans-serif", 30).into_font())
        .margin(25)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..10f64, 5f64..25f64)
        .expect("ChartBuilder failed");

    // Configure mesh with axis labels and grid lines
    chart
        .configure_mesh()
        .x_labels(10) // increments of 1
        .y_labels(5) // increments of 5
        .x_desc(plot_labels.x_axis_label) // Label for the x-axis
        .y_desc(plot_labels.y_axis_label) // Label for the y-axis
        .x_label_style(("sans-serif", 18).into_font())
        .y_label_style(("sans-serif", 18).into_font())
        .x_label_formatter(&|x| format!("{}", *x as i64))
        .y_label_formatter(&|y| format!("{}", *y as i64))
        .draw()
        .expect("configure_mesh() failed");

    // Plot the raw data as a red line, then add points
    chart
        .draw_series(LineSeries::new(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &RED,
        ))
        .expect(
            format!(
                "draw_series() LineSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        );

    chart
        .draw_series(PointSeries::of_element(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x, &y)| (x, y)),
            4, // Size of the points
            &RED,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Cross::new((0, 0), size, style.filled());
            },
        ))
        .expect(
            format!(
                "draw_series() PointSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        )
        .label(plot_labels.x_axis_data1_label)
        .legend(|(x, y)| EmptyElement::at((x + 10, y)) + Cross::new((0, 0), 3, RED.filled()));

    // Plot the filtered data as a blue line, then add points
    chart
        .draw_series(LineSeries::new(
            y_axis_data
                .iter()
                .zip(x_axis_data2.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &BLUE,
        ))
        .expect(
            format!(
                "draw_series() LineSeries {} failed",
                plot_labels.x_axis_data2_label
            )
            .as_str(),
        );

    chart
        .draw_series(PointSeries::of_element(
            y_axis_data
                .iter()
                .zip(x_axis_data2.iter())
                .map(|(&x, &y)| (x, y)),
            3, // Size of the points
            &BLUE,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled());
            },
        ))
        .expect(
            format!(
                "draw_series() PointSeries {} failed",
                plot_labels.x_axis_data2_label
            )
            .as_str(),
        )
        .label(plot_labels.x_axis_data2_label)
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

    println!("Kalman filter plot written: {}", plot_labels.plot_pathname);

    // --- MAKE PLOTS --------------------------------------------------------//
    // Build and save graph using plotters crate; graph format based on textbook example
    let x_axis_data1 = error_covariance_p;

    let plot_labels = PlotLabels {
        plot_pathname: "./plots/04b_KalmanFilter_SimpleEx.png".to_string(),
        title: "Kalman Filter".to_string(),
        x_axis_label: "Time [s]".to_string(),
        y_axis_label: "Error Covariance, P".to_string(),
        x_axis_data1_label: "Measurements".to_string(),
        x_axis_data2_label: "".to_string(),
    };

    let root = BitMapBackend::new(&plot_labels.plot_pathname, (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    // Configure the chart
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_labels.title, ("sans-serif", 30).into_font())
        .margin(25)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..10f64, 0f64..2.5f64)
        .expect("ChartBuilder failed");

    // Configure mesh with axis labels and grid lines
    chart
        .configure_mesh()
        .x_labels(20) // increments of 1
        .y_labels(10) // increments of 0.5
        .x_desc(plot_labels.x_axis_label) // Label for the x-axis
        .y_desc(plot_labels.y_axis_label) // Label for the y-axis
        .x_label_style(("sans-serif", 18).into_font())
        .y_label_style(("sans-serif", 18).into_font())
        .x_label_formatter(&|x| format!("{}", *x as i64))
        .y_label_formatter(&|y| format!("{:.1}", *y as f64))
        .draw()
        .expect("configure_mesh() failed");

    chart
        .draw_series(LineSeries::new(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &BLUE,
        ))
        .expect(
            format!(
                "draw_series() LineSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        );

    chart
        .draw_series(PointSeries::of_element(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x, &y)| (x, y)),
            3, // Size of the points
            &BLUE,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled());
            },
        ))
        .expect(
            format!(
                "draw_series() PointSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        );

    let _ = root.present();

    println!(
        "Kalman filter error covariance plot written: {}",
        plot_labels.plot_pathname
    );

    // --- MAKE PLOTS --------------------------------------------------------//
    // Build and save graph using plotters crate; graph format based on textbook example
    let x_axis_data1 = kalman_gain_k;

    let plot_labels = PlotLabels {
        plot_pathname: "./plots/04c_KalmanFilter_SimpleEx.png".to_string(),
        title: "Kalman Filter".to_string(),
        x_axis_label: "Time [s]".to_string(),
        y_axis_label: "Kalman Gain, K".to_string(),
        x_axis_data1_label: "Measurements".to_string(),
        x_axis_data2_label: "".to_string(),
    };

    let root = BitMapBackend::new(&plot_labels.plot_pathname, (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    // Configure the chart
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_labels.title, ("sans-serif", 30).into_font())
        .margin(25)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..10f64, 0.0f64..0.7f64)
        .expect("ChartBuilder failed");

    // Configure mesh with axis labels and grid lines
    chart
        .configure_mesh()
        .x_labels(20) // increments of 1
        .y_labels(10) // increments of 0.1
        .x_desc(plot_labels.x_axis_label) // Label for the x-axis
        .y_desc(plot_labels.y_axis_label) // Label for the y-axis
        .x_label_style(("sans-serif", 18).into_font())
        .y_label_style(("sans-serif", 18).into_font())
        .x_label_formatter(&|x| format!("{}", *x as i64))
        .y_label_formatter(&|y| format!("{:.1}", *y as f64))
        .draw()
        .expect("configure_mesh() failed");

    chart
        .draw_series(LineSeries::new(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            &BLUE,
        ))
        .expect(
            format!(
                "draw_series() LineSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        );

    chart
        .draw_series(PointSeries::of_element(
            y_axis_data
                .iter()
                .zip(x_axis_data1.iter())
                .map(|(&x, &y)| (x, y)),
            3, // Size of the points
            &BLUE,
            &|coord, size, style| {
                return EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled());
            },
        ))
        .expect(
            format!(
                "draw_series() PointSeries {} failed",
                plot_labels.x_axis_data1_label
            )
            .as_str(),
        );

    let _ = root.present();

    println!(
        "Kalman filter Kalman gain plot written: {}",
        plot_labels.plot_pathname
    );
}
