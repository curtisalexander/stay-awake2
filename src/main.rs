use clap::Parser;
use stay_awake::Args;
use std::process;

fn main() {
    let args = Args::parse();
    if let Err(e) = stay_awake::run(args) {
        println!("Stopping with error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}