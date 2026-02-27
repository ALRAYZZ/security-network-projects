use sysinfo::{Networks, System};
use std::{thread, time::Duration};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;


pub struct NetworkStats {
    pub download_bps: f64,
    pub upload_bps: f64,
    pub download_history: Vec<f64>,
    pub upload_history: Vec<f64>,
}

pub struct NetworkEngine {
    counter: Arc<AtomicU64>,
    last_total: u64,
    last_instant: Instant,
    pub download_history: Vec<f64>
}

impl NetworkEngine {
    pub fn new(counter: Arc<AtomicU64>) -> Self {
        Self {
            counter,
            last_total: 0,
            last_instant: Instant::now(),
            download_history: Vec::with_capacity(60), // Store last 60
        }
    }

    pub fn update(&mut self) ->  NetworkStats {
        let now = Instant::now();
        let current_total = self.counter.load(Ordering::Relaxed);

        let delta_bytes = current_total.saturating_sub(self.last_total);

        let elapsed = now.duration_since(self.last_instant).as_secs_f64();

        let bps = if elapsed > 0.0 {
            delta_bytes as f64 / elapsed
        } else {
            0.0
        };

        self.last_total = current_total;
        self.last_instant = now;

        self.download_history.push(bps);
        if self.download_history.len() > 60 {
            self.download_history.remove(0);
        }

        NetworkStats {
            download_bps: bps,
            upload_bps: 0.0, // Upload not implemented
            download_history: self.download_history.clone(),
            upload_history: vec![],
        }
    }
}
// Change unit based on size of the speed
pub fn human_readable(bytes_per_sec: f64) -> String {
    if bytes_per_sec > 1024.0 * 1024.0 {
        format!("{:.2} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    } else if bytes_per_sec > 1024.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.2} B/s", bytes_per_sec)
    }
}
