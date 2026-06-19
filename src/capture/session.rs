use pcap::{BpfProgram, Capture, Device};
use crossbeam::channel::Sender;

use crate::config::CaptureConfig;

use crate::types::packetdata::PacketData;

pub fn start(config : &CaptureConfig, tx : Sender<PacketData>) -> Result<(), pcap::Error> {

    let mut cap = Capture::from_device(config.device.as_str())?
        .snaplen(config.snaplen)
        .promisc(config.promisc)
        .timeout(config.timeout)
        .open()?;
   
    cap.filter(config.filter.as_str(), true)?;

    println!("Starting capture");
    while let Ok(packet) = cap.next_packet() {
        println!("Got packet: {} bytes", packet.len());
        tx.send(PacketData {
            header: packet.header.clone(),
            data: packet.data.to_vec(),
        }).unwrap();  
    }

    Ok(()) 
}
