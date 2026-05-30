use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;

use crate::config::CaptureConfig;

use crate::types::packetdata::PacketData;


pub fn output(config : &CaptureConfig, rx : Receiver<Vec<u8>>) {

       loop {} 
}
