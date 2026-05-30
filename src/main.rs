use clap::Parser;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crossbeam::channel::unbounded;
use std::thread;

mod capture;
mod cli;
mod config;
mod network;
mod output;
mod parse;
mod types;

use types::packetdata::PacketData;

fn main() -> Result<(), pcap::Error> {
    println!("{}", output::motd::motd());

    let args = cli::Args::parse();

    //    [capture thread] --channel--> [parse thread] --channel--> [output thread]
    //      raw bytes                  ParsedPacket                 display/file/json

    // Handles data between capturing and parsing
    let (raw_tx, raw_rx): (Sender<PacketData>, Receiver<PacketData>) = unbounded();

    // Handle data between parsed packets to output formats
    let (parsed_tx, parsed_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();

    let config_args = cli::Args::into_config(args);

    let work1 = thread::spawn({
        let config = config_args.clone();
        move || capture::session::start(&config, raw_tx).unwrap()
    });

    let work2 = thread::spawn({
        let config = config_args.clone();
        move || parse::parse::parse(&config, raw_rx, parsed_tx)
    });

    let work3 = thread::spawn({
        let config = config_args.clone();
        move || output::output::output(&config, parsed_rx)
    });


    work1.join().unwrap();
    work2.join().unwrap();
    work3.join().unwrap();

    Ok(())
}
