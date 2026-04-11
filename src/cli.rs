use clap::Parser;

#[derive(Parser)]
#[command(name = "observe", about = "A packet capture tool")]
pub struct Args {
    #[arg(short, long, default_value = "eth0")]
    interface: String,

    #[arg(short, long, default_value_t = 0)]
    count: usize,

    #[arg(short, long)]
    filter: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}
