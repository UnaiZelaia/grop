#![allow(unused)]

use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::Display;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}



fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path).expect("Unable to open");
    let bufer = BufReader::new(f);

    
    for line in bufer.lines() {
        let line = line.unwrap();
        if (line.contains(&args.pattern)) {
            println!("{}", line);
        }
    }
}
