use clap::Parser;

mod capture;
mod cli;
mod output;

fn main() {
    println!("{}", output::motd::motd());
    
    let args = cli::Args::parse();

    println!("{}", args.interface);
}
