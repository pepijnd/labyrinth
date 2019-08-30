use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Counter {
    counts: u64,
    times: VecDeque<Duration>,
    rates: VecDeque<Instant>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            counts: 0,
            times: VecDeque::new(),
            rates: VecDeque::new(),
        }
    }

    pub fn get_times(&self) -> Vec<Duration> {
        Vec::from(self.times.clone())
    }

    pub fn get_rate(&self) -> f32 {
        let rates = self.get_rates();
        rates.iter().sum::<f32>() / rates.len() as f32
    }

    pub fn get_rates(&self) -> Vec<f32> {
        self.rates
            .iter()
            .zip(self.rates.iter().skip(1))
            .map(|(a, b)| 1.0 / (*a - *b).as_secs_f32())
            .collect::<Vec<f32>>()
    }

    pub fn get_counts(&self) -> u64 {
        self.counts
    }

    pub fn count<F>(&mut self, func: F)
    where
        F: Fn(),
    {
        let now = Instant::now();
        func();
        self.times.push_front(now.elapsed());
        self.rates.push_front(Instant::now());
        self.times.truncate(10);
        self.rates.truncate(10);
        self.counts += 1;
    }
}
