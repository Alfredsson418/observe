use pcap::{Capture, Device};
use crossbeam::channel::Sender;

use crate::config::CaptureConfig;

pub fn start(config : &CaptureConfig, tx : Sender<Vec<u8>>) {
    /*
    Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();  
    */
}
