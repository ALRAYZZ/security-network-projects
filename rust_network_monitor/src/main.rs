use sysinfo::{Networks, System};
use std::{thread, time::Duration};
use std::collections::VecDeque;
use std::time::Instant;

// Change unit based on size of the speed
fn human_readable(bytes_per_sec: f64) -> String {
    if bytes_per_sec > 1024.0 * 1024.0 {
        format!("{:.2} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    } else if bytes_per_sec > 1024.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.2} B/s", bytes_per_sec)
    }
}


fn main() {
    // Ask OS what network interfaces exist
    // Networks is a struct that holds the data of all network interfaces
    let mut networks = Networks::new_with_refreshed_list();

    // Hardcoded interface FOR TESTING
    let interface_name = "Ethernet 3";

    // Initial refresh
    // Refreshing the data to get the initial values for the selected interface
    networks.refresh();

    // Get the initial received and transmitted bytes for the selected interface
    let mut previous_rx = networks
        .get(interface_name)
        .expect("Interface not found")
        .received();

    let mut previous_tx = networks
        .get(interface_name)
        .expect("Interface not found")
        .transmitted();

    // Get the time we got the data
    let mut previous_time = Instant::now();


    // Rolling buffer for smoothing the speed values
    let mut rx_history: VecDeque<f64> = VecDeque::with_capacity(5);
    let mut tx_history: VecDeque<f64> = VecDeque::with_capacity(5);


    loop  {
        thread::sleep(Duration::from_secs(1));
        networks.refresh();

        let now = Instant::now();
        let elapsed = now.duration_since(previous_time).as_secs_f64();

        let data = networks
            .get(interface_name)
            .expect("Interface not found");

        // Recalculate the received and transmitted bytes for the selected interface
        let current_rx = data.received();
        let current_tx = data.transmitted();

        // Calculate the difference in received and transmitted bytes since the last check
        // Use saturating_sub to avoid negative values in case of counter reset (underflow)
        let delta_rx = current_rx.saturating_sub(previous_rx) as f64 / elapsed;
        let delta_tx = current_tx.saturating_sub(previous_tx) as f64 / elapsed;

        // Add to history
        if rx_history.len() == 5 { rx_history.pop_front(); }
        if tx_history.len() == 5 { tx_history.pop_front(); }
        rx_history.push_back(delta_rx);
        tx_history.push_back(delta_tx);

        // Calculate average speed over the history
        let rx_avg: f64 = rx_history.iter().sum::<f64>() / rx_history.len() as f64;
        let tx_avg: f64 = tx_history.iter().sum::<f64>() / tx_history.len() as f64;

        println!("Download: {}", human_readable(rx_avg));
        println!("Upload: {}", human_readable(tx_avg));
        println!("---------------------------------");

        previous_rx = current_rx;
        previous_tx = current_tx;
        previous_time = now;
    }
}
