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
