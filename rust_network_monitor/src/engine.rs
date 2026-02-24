use sysinfo::{Networks, System};
use std::{thread, time::Duration};
use std::collections::VecDeque;
use std::time::Instant;


pub struct NetworkStats {
    pub download_bps: f64,
    pub upload_bps: f64,
}

pub struct NetworkEngine {
    interface_name: String,
    networks: Networks,
    previous_rx: u64,
    previous_tx: u64,
    previous_time: Instant,
    rx_history: VecDeque<f64>,
    tx_history: VecDeque<f64>,
}

impl NetworkEngine {
    pub fn new(interface_name: &str) -> Self {
        // Ask OS what network interfaces exist
        // Networks is a struct that holds the data of all network interfaces
        let mut networks = Networks::new_with_refreshed_list();

        // Initial refresh
        // Refreshing the data to get the initial values for the selected interface
        networks.refresh();

        let data = networks
            .get(interface_name)
            .expect("Interface not found");

        Self {
            interface_name: interface_name.to_string(),
            previous_rx: data.received(),
            previous_tx: data.transmitted(),
            networks, // Return networks here after data ends it work since it uses networks to get received and transmitted data
            previous_time: Instant::now(),
            rx_history: VecDeque::with_capacity(5),
            tx_history: VecDeque::with_capacity(5),
        }
    }

    pub fn update(&mut self) ->  NetworkStats {
        self.networks.refresh();

        let now = Instant::now();
        let elapsed = now.duration_since(self.previous_time).as_secs_f64();

        let data = self
            .networks
            .get(&self.interface_name)
            .expect("Interface not found");

        // Recalculate the received and transmitted bytes for the selected interface
        let current_rx = data.received();
        let current_tx = data.transmitted();

        // Calculate the difference in received and transmitted bytes since the last check
        // Use saturating_sub to avoid negative values in case of counter reset (underflow)
        let delta_rx = current_rx.saturating_sub(self.previous_rx) as f64 / elapsed;
        let delta_tx = current_tx.saturating_sub(self.previous_tx) as f64 / elapsed;

        // Add to history
        if self.rx_history.len() == 5 { self.rx_history.pop_front(); }
        if self.tx_history.len() == 5 { self.tx_history.pop_front(); }
        self.rx_history.push_back(delta_rx);
        self.tx_history.push_back(delta_tx);

        // Calculate average speed over the history
        let rx_avg: f64 = self.rx_history.iter().sum::<f64>() / self.rx_history.len() as f64;
        let tx_avg: f64 = self.tx_history.iter().sum::<f64>() / self.tx_history.len() as f64;

        self.previous_rx = current_rx;
        self.previous_tx = current_tx;
        self.previous_time = now;

        NetworkStats {
            download_bps: rx_avg,
            upload_bps: tx_avg,
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
