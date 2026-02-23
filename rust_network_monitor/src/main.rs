use sysinfo::{Networks, System};
use std::{thread, time::Duration};
use std::time::Instant;

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
        let delta_rx = current_rx.saturating_sub(previous_rx);
        let delta_tx = current_tx.saturating_sub(previous_tx);

        // Calculate the download and upload speeds in bytes per second
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
