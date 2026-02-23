use sysinfo::{Networks, System};
use std::{thread, time::Duration};


fn main() {
    let mut networks = Networks::new_with_refreshed_list();

    loop  {
        networks.refresh();

        for (interface_name, data) in &networks {
            println!("Interface: {}", interface_name);
            println!("  Received: {} bytes", data.received());
            println!("  Transmitted: {} bytes", data.transmitted());
        }

        println!("---------------------------------");

        thread::sleep(Duration::from_secs(1));
    }
}
