mod parser;
mod interactive;

use std::io;
use std::io::{Read};
use clap::Parser;
use crate::parser::parser::{Args, parse_streams, ParserSettings};


fn main() -> Result<(), io::Error> {
    println!("Current used OS: {}", std::env::consts::OS);
    let args = Args::parse();
    if !args.interactive {
        let streams = parse_streams(ParserSettings::from_args(args)?)?;
        println!("{:?}", streams);
        return Ok(());
    }

    Ok(())
}
