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
