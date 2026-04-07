use pcap::Device;

pub fn get_device() -> pcap::Device {
    Device::lookup().unwrap().unwrap()
}

/*pub fn get_all_devices() -> pcap::Device {
    
}
*/
