use pcap::{Device, Capture};

pub fn run_sniffer(interface_name: &str) {
    // Ask pcap to list all network interfaces and find the one that matches the provided name
    let device = Device::list()
        .expect("Failed to list devices")
        .into_iter()
        .find(|d| d.name == interface_name)
        .expect("Interface not found");

    // Create a capture handle for the selected interface
    // Promisc captures all packets on NIC, immediate packets delivered instantly without buffering
    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .immediate_mode(true)
        .open()
        .unwrap();

    println!("Listening on {}", interface_name);

    // Capture packets in a loop and print their lengths
    while let Ok(packet) = cap.next_packet() {
        println!("Packet captured: {} bytes", packet.data.len());
    }
}