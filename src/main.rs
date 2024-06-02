pub mod kalman_filter_test;
pub mod recursive_filters_test;
pub mod sensor_spoofs;
pub mod utils;

use crate::kalman_filter_test::{
    kalman_filter_estimate_position_with_velocity_example,
    kalman_filter_estimate_velocity_from_position_example, kalman_filter_extremely_simple_example,
    kalman_filter_measure_velocity_with_sonar_example,
};
use crate::recursive_filters_test::{
    average_filter_example, first_order_low_pass_filter_example, moving_average_filter_example,
};

fn main() {
    // Recursive Filters
    average_filter_example();
    moving_average_filter_example();
    first_order_low_pass_filter_example();

    // Kalman Filter
    kalman_filter_extremely_simple_example();

    kalman_filter_estimate_velocity_from_position_example();
    kalman_filter_estimate_position_with_velocity_example();
    kalman_filter_measure_velocity_with_sonar_example();
}
