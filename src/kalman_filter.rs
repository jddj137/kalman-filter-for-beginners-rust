#![allow(non_snake_case)]

use nalgebra::{DMatrix, DVector};

/// Based on definition in Chapter 8, pg. 66
/// State
/// state_transition_mat_A: DMatrix<f64>, // n x n matrix
/// Process noise covariance matrix
/// covariance_mat_state_transition_noise_Q: DMatrix<f64>, // n x n diagonal matrix
/// Measurement
/// Observation matrix
/// state_to_measurement_mat_H: DMatrix<f64>, // m x n matrix
/// Measurement noise covariance matrix
/// covariance_mat_measurement_noise_R: DMatrix<f64>,  // m x m diagonal matrix
pub struct SystemModel {
    // State
    st_trns_A: DMatrix<f64>,
    prcs_cvr_Q: DMatrix<f64>,
    // Measurement
    st_to_meas_H: DMatrix<f64>,
    meas_cvr_R: DMatrix<f64>,
}

impl SystemModel {
    pub fn new(A: DMatrix<f64>, Q: DMatrix<f64>, H: DMatrix<f64>, R: DMatrix<f64>) -> Self {
        // TODO: Input validation to verify matrix dimensions

        Self {
            st_trns_A: A,
            prcs_cvr_Q: Q,
            st_to_meas_H: H,
            meas_cvr_R: R,
        }
    }
}

/// Prediction
/// state_pred_x: DVector<f64>, // n x 1 column vector
/// err_covar_est_P: DMatrix<f64>, // n x n matrix
/// Estimation
/// state_est_x: DVector<f64>,  // n x 1 column vector
/// err_covar_pred_P: DMatrix<f64>, // n x n matrix
/// kalman_gain_K: DMatrix<f64>, // n x m matrix
pub struct KalmanFilter {
    sys_model: SystemModel,
    // Prediction
    prd_x: DVector<f64>,
    prd_cvr_P: DMatrix<f64>,
    // Kalman Gain
    klmn_gain_K: DMatrix<f64>,
    // Estimation
    est_x: DVector<f64>,
    est_cvr_P: DMatrix<f64>,
}

impl KalmanFilter {
    pub fn new(
        system_model: SystemModel,
        initial_est_state_x: DVector<f64>,
        initial_est_covar_P: DMatrix<f64>,
    ) -> Self {
        // TODO: Input validation to verify matrix dimensions

        // Initialize predictions and Kalman gain based on initial inputs
        // step 1.a
        let prd_x = Self::predict_state(&initial_est_state_x, &system_model.st_trns_A);
        // step 1.b
        let prd_cvr_P = Self::predict_error_covariance(
            &initial_est_covar_P,
            &system_model.st_trns_A,
            &system_model.prcs_cvr_Q,
        );
        // step 2
        let klmn_gain_K = Self::calculate_kalman_gain(
            &prd_cvr_P,
            &system_model.st_to_meas_H,
            &system_model.meas_cvr_R,
        );

        Self {
            sys_model: system_model,
            prd_x,
            prd_cvr_P,
            klmn_gain_K,
            est_x: initial_est_state_x,
            est_cvr_P: initial_est_covar_P,
        }
    }

    // All equations based on Figure 5.1
    // State Prediction
    fn predict_state(prev_est_x: &DVector<f64>, state_trns_A: &DMatrix<f64>) -> DVector<f64> {
        state_trns_A * prev_est_x
    }

    // Error Covariance Prediction
    fn predict_error_covariance(
        prev_est_cvr_P: &DMatrix<f64>,
        state_trns_A: &DMatrix<f64>,
        prcs_cvr_Q: &DMatrix<f64>,
    ) -> DMatrix<f64> {
        state_trns_A * prev_est_cvr_P * state_trns_A.transpose() + prcs_cvr_Q
    }

    // Kalman Gain
    fn calculate_kalman_gain(
        prd_cvr_P: &DMatrix<f64>,
        state_to_meas_H: &DMatrix<f64>,
        meas_cvr_R: &DMatrix<f64>,
    ) -> DMatrix<f64> {
        let denom: DMatrix<f64> =
            state_to_meas_H * prd_cvr_P * state_to_meas_H.transpose() + meas_cvr_R;
        prd_cvr_P
            * state_to_meas_H.transpose()
            * denom
                .try_inverse()
                .expect("calculate_kalman_gain(): Matrix is not invertible")
    }

    // State Estimate
    // measurement_z is m x 1 column vector
    fn estimate_state(
        meas_z: DVector<f64>,
        prd_x: &DVector<f64>,
        klmn_gain_K: &DMatrix<f64>,
        state_to_meas_H: &DMatrix<f64>,
    ) -> DVector<f64> {
        prd_x + klmn_gain_K * (meas_z - state_to_meas_H * prd_x)
    }

    // Error Covariance Estimate
    fn estimate_error_convariance(
        prd_cvr_P: &DMatrix<f64>,
        klmn_gain_K: &DMatrix<f64>,
        state_to_meas_H: &DMatrix<f64>,
    ) -> DMatrix<f64> {
        prd_cvr_P - klmn_gain_K * state_to_meas_H * prd_cvr_P
    }

    pub fn update(&mut self, measurement_z: DVector<f64>) {
        // step 3
        self.est_x = Self::estimate_state(
            measurement_z,
            &self.prd_x,
            &self.klmn_gain_K,
            &self.sys_model.st_to_meas_H,
        );
        // step 4
        self.est_cvr_P = Self::estimate_error_convariance(
            &self.prd_cvr_P,
            &self.klmn_gain_K,
            &self.sys_model.st_to_meas_H,
        );
        // step 1.a
        self.prd_x = Self::predict_state(&self.est_x, &self.sys_model.st_trns_A);
        // step 1.b
        self.prd_cvr_P = Self::predict_error_covariance(
            &self.est_cvr_P,
            &self.sys_model.st_trns_A,
            &self.sys_model.prcs_cvr_Q,
        );
        // step 2
        self.klmn_gain_K = Self::calculate_kalman_gain(
            &self.prd_cvr_P,
            &self.sys_model.st_to_meas_H,
            &self.sys_model.meas_cvr_R,
        );
    }

    pub fn get_state_estimate(&self) -> DVector<f64> {
        self.est_x.clone()
    }

    pub fn get_error_covariance(&self) -> DMatrix<f64> {
        self.est_cvr_P.clone()
    }

    pub fn get_kalman_gain(&self) -> DMatrix<f64> {
        self.klmn_gain_K.clone()
    }
}
