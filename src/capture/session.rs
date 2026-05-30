use pcap::{Capture, Device};
use crossbeam::channel::Sender;

use crate::config::CaptureConfig;

pub fn start(config : &CaptureConfig, tx : Sender<Vec<u8>>) -> Result<(), pcap::Error> {

    let mut cap = Capture::from_device(config.interface.as_str())?
        .snaplen(config.snaplen)
        .promisc(config.promisc)
        .timeout(config.timeout)
        .open()?;
    
    println!("Starting capture");
    while let Ok(packet) = cap.next_packet() {
        println!("Got packet: {} bytes", packet.len());
    }


    Ok(()) 
} 
