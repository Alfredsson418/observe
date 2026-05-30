use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;

use crate::config::CaptureConfig;

use crate::types::packetdata::PacketData;


pub fn parse(config : &CaptureConfig, rx : Receiver<PacketData>, tx : Sender<Vec<u8>>) {

       loop {} 
}
