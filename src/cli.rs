use clap::Parser;

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
