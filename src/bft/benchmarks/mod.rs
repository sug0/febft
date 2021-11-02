use std::time::{Instant, SystemTime};

pub struct RequestMeta {
    reception_time: SystemTime,
    consensus_start_time: SystemTime,
    consensus_decision_time: SystemTime,
    execution_time: SystemTime,
}

pub struct BenchmarkHelper {
    values: Vec<i64>,
}

pub trait BenchmarkHelperStore {
    fn store(self, bench: &mut BenchmarkHelper);
}

// this code was more or less 1:1 translated from BFT-SMaRt,
// even its oddities, such as cloning the values array
impl BenchmarkHelper {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn reset(&mut self) {
        self.values.clear();
    }

    pub fn average(&self, percent: bool) -> f64 {
        let mut values = self.values.clone();
        let limit = if percent { values.len() / 10 } else { 0 };

        values.sort_unstable();

        let count: i64 = (&values[limit..(values.len() - limit)])
            .iter()
            .sum();

        (count as f64) / ((values.len() - 2*limit) as f64)
    }
}

impl BenchmarkHelperStore for (SystemTime, SystemTime) {
    fn store(self, bench: &mut BenchmarkHelper) {
        const MAX: u128 = i64::MAX as u128;

        let (start, end) = self;
        let duration = end
            .duration_since(start)
            .expect("Non-monotonic time detected!")
            .as_nanos();

        bench.values.push((duration & MAX) as i64);
    }
}

impl BenchmarkHelperStore for (Instant, Instant) {
    fn store(self, bench: &mut BenchmarkHelper) {
        const MAX: u128 = i64::MAX as u128;

        let (start, end) = self;
        let duration = end
            .duration_since(start)
            .as_nanos();

        bench.values.push((duration & MAX) as i64);
    }
}

impl BenchmarkHelperStore for usize {
    fn store(self, bench: &mut BenchmarkHelper) {
        const MAX: usize = i64::MAX as usize;

        bench.values.push((self & MAX) as i64);
    }
}
