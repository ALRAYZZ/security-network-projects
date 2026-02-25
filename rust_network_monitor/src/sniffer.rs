use pcap::{Device, Capture};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread;



pub fn start_packet_counter(interface_name: &str) -> Arc<AtomicU64> {
    let byte_counter = Arc::new(AtomicU64::new(0));
    let counter_clone = Arc::clone(&byte_counter);
    let interface = interface_name.to_string();

    // Packet capturing blocks main thread, so we run it in a separate thread
    thread::spawn(move || {
        let device = Device::list()
            .expect("Failed to list devices")
            .into_iter()
            .find(|d| d.name == interface)
            .expect("Interface not found");

        let mut cap = Capture::from_device(device)
            .unwrap()
            .promisc(true)
            .immediate_mode(true)
            .open()
            .unwrap();

        while let Ok(packet) = cap.next_packet() {
            counter_clone.fetch_add(packet.data.len() as u64, Ordering::Relaxed);
        }
    });

    byte_counter
}