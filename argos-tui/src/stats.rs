use std::collections::VecDeque;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub pid: u32,
    pub name: String,
    pub cpu_history: VecDeque<f64>,
    pub memory_history: VecDeque<f64>,
    pub disk_read_history: VecDeque<f64>,
    pub disk_write_history: VecDeque<f64>,
    pub timestamps: VecDeque<Instant>,
    pub max_history: usize,
}

#[derive(Debug, Clone)]
pub struct StatisticalMetrics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub trend: f64, // Pendiente de la regresión lineal
    pub forecast: f64, // Predicción próximo valor
    pub percentile_95: Option<f64>, // Percentil 95
    pub skewness: Option<f64>, // Sesgo (skewness)
}

impl ProcessStats {
    pub fn new(pid: u32, name: String, max_history: usize) -> Self {
        Self {
            pid,
            name,
            cpu_history: VecDeque::with_capacity(max_history),
            memory_history: VecDeque::with_capacity(max_history),
            disk_read_history: VecDeque::with_capacity(max_history),
            disk_write_history: VecDeque::with_capacity(max_history),
            timestamps: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    pub fn add_sample(&mut self, cpu: f64, memory: f64, disk_read: f64, disk_write: f64) {
        let now = Instant::now();
        
        self.cpu_history.push_back(cpu);
        self.memory_history.push_back(memory);
        self.disk_read_history.push_back(disk_read);
        self.disk_write_history.push_back(disk_write);
        self.timestamps.push_back(now);

        if self.cpu_history.len() > self.max_history {
            self.cpu_history.pop_front();
            self.memory_history.pop_front();
            self.disk_read_history.pop_front();
            self.disk_write_history.pop_front();
            self.timestamps.pop_front();
        }
    }

    pub fn calculate_metrics(&self, data: &VecDeque<f64>) -> StatisticalMetrics {
        if data.is_empty() {
            return StatisticalMetrics {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                trend: 0.0,
                forecast: 0.0,
                percentile_95: None,
                skewness: None,
            };
        }

        let values: Vec<f64> = data.iter().copied().collect();
        let n = values.len() as f64;

        // Cálculos básicos
        let mean = values.iter().sum::<f64>() / n;
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // Desviación estándar
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / n;
        let std_dev = variance.sqrt();

        // Mediana
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = if n % 2.0 == 0.0 {
            (sorted[(n as usize / 2) - 1] + sorted[n as usize / 2]) / 2.0
        } else {
            sorted[n as usize / 2]
        };

        // Percentil 95
        let percentile_95 = if !sorted.is_empty() {
            let index = ((0.95 * n).ceil() as usize).min(sorted.len() - 1);
            Some(sorted[index])
        } else {
            None
        };

        // Sesgo (skewness)
        let skewness = if std_dev > 0.0 && n > 2.0 {
            let skew = values.iter()
                .map(|x| ((x - mean) / std_dev).powi(3))
                .sum::<f64>() / n;
            Some(skew)
        } else {
            None
        };

        // Regresión lineal para tendencia
        let (trend, forecast) = self.linear_regression(&values);

        StatisticalMetrics {
            mean,
            median,
            std_dev,
            min,
            max,
            trend,
            forecast,
            percentile_95,
            skewness,
        }
    }

    fn linear_regression(&self, values: &[f64]) -> (f64, f64) {
        let n = values.len() as f64;
        if n < 2.0 {
            return (0.0, *values.last().unwrap_or(&0.0));
        }

        let x_mean = (n - 1.0) / 2.0;
        let y_mean = values.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &y) in values.iter().enumerate() {
            let x = i as f64 - x_mean;
            numerator += x * (y - y_mean);
            denominator += x * x;
        }

        let slope = if denominator == 0.0 { 0.0 } else { numerator / denominator };
        let next_value = y_mean + slope * (x_mean + 1.0);

        (slope, next_value)
    }

    pub fn get_cpu_metrics(&self) -> StatisticalMetrics {
        self.calculate_metrics(&self.cpu_history)
    }

    pub fn get_memory_metrics(&self) -> StatisticalMetrics {
        self.calculate_metrics(&self.memory_history)
    }

    pub fn get_disk_read_metrics(&self) -> StatisticalMetrics {
        self.calculate_metrics(&self.disk_read_history)
    }

    pub fn get_disk_write_metrics(&self) -> StatisticalMetrics {
        self.calculate_metrics(&self.disk_write_history)
    }

    pub fn get_time_window(&self) -> Option<Duration> {
        if self.timestamps.len() < 2 {
            return None;
        }
        Some(self.timestamps.back()?.duration_since(*self.timestamps.front()?))
    }
}