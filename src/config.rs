#[derive(Clone)]
pub struct CaptureConfig {
    // General arguments
    pub verbose: bool,

    // Capture arguments
    pub device: String,
    pub filter: String,
    pub count: usize,
    pub promisc: bool,
    pub snaplen: i32,
    pub timeout: i32,

    // Other arguments
    pub get_devices: bool, // Display all available devices
}
