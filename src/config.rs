pub struct CaptureConfig {
    pub interface: String,
    pub filter: Option<String>,
    pub count: usize,
    pub verbose: bool,
}
