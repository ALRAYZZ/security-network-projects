use sysinfo::{Networks, System};
use std::{thread, time::Duration};
use std::time::Instant;

fn main() {
    // Ask OS what network interfaces exist
    let mut networks = Networks::new_with_refreshed_list();

    // Hardcoded interface FOR TESTING
    let interface_name = "Ethernet 3";

    // Initial refresh
    // Reset the data of all interfaces to 0, and ask OS to update them
    networks.refresh();

    let mut previous_rx = networks
        .get(interface_name)
        .expect("Interface not found")
        .received();

    let mut previous_tx = networks
        .get(interface_name)
        .expect("Interface not found")
        .transmitted();

    let mut previous_time = Instant::now();

    loop  {
        thread::sleep(Duration::from_secs(1));
        networks.refresh();

        let now = Instant::now();
        let elapsed = now.duration_since(previous_time).as_secs_f64();

        let data = networks
            .get(interface_name)
            .expect("Interface not found");

        let current_rx = data.received();
        let current_tx = data.transmitted();

        let delta_rx = current_rx.saturating_sub(previous_rx);
        let delta_tx = current_tx.saturating_sub(previous_tx);

        let rx_per_sec = delta_rx as f64 / elapsed;
        let tx_per_sec = delta_tx as f64 / elapsed;

        println!("Download: {:.2} bytes/s", rx_per_sec);
        println!("Upload: {:.2} bytes/s", tx_per_sec);
        println!("---------------------------------");

        previous_rx = current_rx;
        previous_tx = current_tx;
        previous_time = now;
    }
}
