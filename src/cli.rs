use clap::Parser;
use crate::config::CaptureConfig;

#[derive(Parser)]
#[command(name = "observe", about = "A packet capture tool")]
pub struct Args {
    #[arg(short, long, default_value = "lo")]
    pub interface: String,

    #[arg(short, long, default_value_t = 0)]
    pub count: usize,

    #[arg(short, long)]
    pub filter: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(long, default_value_t = 65535)]
    pub snaplen: i32,

    #[arg(long, default_value_t = true)]
    pub promisc: bool,

    #[arg(short, long, default_value_t = 1000)]
    pub timeout: i32,
}

impl Args {
    pub fn into_config(self) -> CaptureConfig {
        CaptureConfig {
            interface: self.interface,
            filter: self.filter,
            count: self.count,
            verbose: self.verbose,
            snaplen: self.snaplen,
            promisc: self.promisc,
            timeout: self.timeout,
        }
    }
}
