pub struct AverageFilter {
    avg: f64,
    total_data_pts: usize, // 'k' in formula
}

impl AverageFilter {
    pub fn new() -> Self {
        Self {
            avg: 0.0,
            total_data_pts: 0,
        }
    }

    pub fn update(&mut self, data: f64) {
        self.total_data_pts += 1;

        let alpha = 1.0 - 1.0 / self.total_data_pts as f64;
        self.avg = alpha * self.avg + (1.0 - alpha) * data;
    }

    pub fn get_average(&self) -> f64 {
        self.avg
    }
}

pub struct MovingAverageFilter {
    avg: f64,
    window_size: usize, // 'n' in formula
    data_buff: Option<Vec<f64>>,
    oldest_data_idx: usize,
}

impl MovingAverageFilter {
    pub fn new(window_size: usize) -> Self {
        assert!(
            window_size > 0,
            "MovingAverageFilter: Window size must be greater than zero."
        );

        Self {
            avg: 0.0,
            window_size,
            data_buff: None,
            oldest_data_idx: 0,
        }
    }

    pub fn update(&mut self, data: f64) {
        if let Some(ref mut data_buff) = self.data_buff {
            // Calculate rolling average
            let oldest_data = data_buff[self.oldest_data_idx];
            self.avg = self.avg + (data - oldest_data) / self.window_size as f64;

            // Book keeping to remove oldest and add newest data point to buffer
            data_buff[self.oldest_data_idx] = data;
            self.oldest_data_idx = (self.oldest_data_idx + 1) % self.window_size;
        } else {
            // Inititially there are not enough data points for rolling average
            // This implementation initializes the buffer with the first data value
            self.data_buff = Some(vec![data; self.window_size]);
            self.avg = data;
        }
    }

    pub fn get_average(&self) -> f64 {
        self.avg
    }
}

pub struct LowPassFilter1stOrder {
    avg: f64,
    alpha: f64,
    initialized: bool,
}

impl LowPassFilter1stOrder {
    pub fn new(alpha: f64) -> Self {
        assert!(
            (alpha > 0.0) && (alpha < 1.0),
            "LowPassFilter1stOrder: Smoothing factor, alpha,
             must be between zero and one, 0.0 < alpha < 1.0"
        );

        Self {
            avg: 0.0,
            alpha,
            initialized: false,
        }
    }

    pub fn update(&mut self, data: f64) {
        if !(self.initialized) {
            self.avg = data;
            self.initialized = true;
        }

        self.avg = self.alpha * self.avg + (1.0 - self.alpha) * data;
    }

    pub fn get_average(&self) -> f64 {
        self.avg
    }
}

/**
 * The generic form of the equation used in the averaging filter and 1st order low pass filter.
 *
 * Alpha is called the smoothing factor.
 * A larger alpha gives more weight to recent data, resulting in a faster response to changes;
 * leads to less smoothing and more sensitivity to recent values.
 * A smaller alpha gives more weight to past data, resulting in a slower response to changes,
 * but leading to smoother outputs and more memory of past values.
 * .
 */
fn exponential_smoothing(data: f64, previous_average: f64, alpha: f64) -> f64 {
    assert!(
        (alpha > 0.0) && (alpha < 1.0),
        "exponential_smoothing: Smoothing factor, alpha,
         must be between zero and one, 0.0 < alpha < 1.0"
    );

    alpha * previous_average + (1.0 - alpha) * data
}
