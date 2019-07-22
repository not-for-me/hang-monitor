use std::{env,  process};

use hang_monitor;
use hang_monitor::Config;

fn main() {
    println!("Hang Monitor");

    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    hang_monitor::run(conf);
}
