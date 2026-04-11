use pcap::{Capture, Device};

pub fn capture_from(device: pcap::Device) {
    Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();  
}
