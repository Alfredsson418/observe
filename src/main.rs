use clap::Parser;
use crossbeam::channel::unbounded;
use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;
use std::thread;

mod capture;
mod cli;
mod output;
mod config;

fn main() {
    println!("{}", output::motd::motd());
    
    let args = cli::Args::parse();
    
    //    [capture thread] --channel--> [parse thread] --channel--> [output thread]
    //      raw bytes                  ParsedPacket                 display/file/json


    // Handles data between capturing and parsing
    let (raw_tx, raw_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();
    
    // Handle data between parsed packets to output formats
    // let (parsed_tx, parsed_rx) : Sender<Vec<u8>, Receiver<Vec<u8>>) = unbounded();
   
    let config_args = cli::Args::into_config(args);

    // Worker 1 — capture
    thread::spawn(move || {
        capture::session::start(&config_args, raw_tx);
    });
}
