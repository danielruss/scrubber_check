use clap::Parser;
use std::{fs::File, io::Read};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    scrubbed_filepath : String,
    original_filepath : String
}
fn main() -> Result<(),String>{
    let args = Args::parse();

    let mut scrubbed_file = File::open(args.scrubbed_filepath).map_err(|e| format!("Scrubbed file: {}",e.to_string()))?;
    let mut original_file = File::open(args.original_filepath).map_err(|e| format!("Original file: {}",e.to_string()))?;
    let mut scrubbed_text = String::new();scrubbed_file.read_to_string(&mut scrubbed_text).map_err(|e| format!("Scrubbed text: {}",e.to_string()))?;
    let mut original_text = String::new();original_file.read_to_string(&mut original_text).map_err(|e| format!("Original file: {}",e.to_string()))?;
    
    let results = scrubber_check::compare(&scrubbed_text, &original_text);
    results.iter().for_each(|s| println!("{}",s));
    Ok(())
}
