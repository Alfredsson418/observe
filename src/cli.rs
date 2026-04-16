use clap::Parser;
use crate::config::CaptureConfig;

#[derive(Parser)]
#[command(name = "observe", about = "A packet capture tool")]
pub struct Args {
    #[arg(short, long, default_value = "eth0")]
    pub interface: String,

    #[arg(short, long, default_value_t = 0)]
    pub count: usize,

    #[arg(short, long)]
    pub filter: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn into_config(self) -> CaptureConfig {
        CaptureConfig {
            interface: self.interface,
            filter: self.filter,
            count: self.count,
            verbose: self.verbose,
        }
    }
}
