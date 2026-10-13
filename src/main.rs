use clap::Parser;
use srt_editor::parser::SubRip;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Location of the SRT file on disk.
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let parser = SubRip::from_string(String::from(""));
    println!("Hello {} !", args.filepath);
}
