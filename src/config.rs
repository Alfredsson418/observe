pub struct CaptureConfig {
    pub interface: String,
    pub filter: Option<String>,
    pub count: usize,
    pub verbose: bool,
    pub promisc: bool,
    pub snaplen: i32,
    pub timeout: i32,
}
