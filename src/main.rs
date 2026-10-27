use std::fs;
use clap::Parser;

mod subrip;
use crate::subrip::SubRip;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Location of input SRT
    #[arg(short, long)]
    input: String,

    /// Location of the output SRT
    #[arg(short, long)]
    output: String,

    /// Offset in sec
    #[arg(long, default_value_t = 0f64)]
    offset: f64
    
}

fn main() {
    let Args {input, output, offset} = Args::parse();
    let parser = SubRip::from_string(String::from(""));
    println!("Reading original SRT from location{}", input);
    let content = fs::read_to_string(input).unwrap();
    let subrip = SubRip::from_string(content).unwrap();
    
}
