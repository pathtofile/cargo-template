use clap::Parser;
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// Setup Commandline args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config
    #[arg(short, long, default_value = "example.json")]
    config: String,
}

// Do stuff
fn do_stuff(data: &Value) -> Result<(), Box<dyn Error>> {
    let name = data["name"].as_str().ok_or("Config missing 'name'")?;
    println!("Name: {name}");

    let resp: Value = reqwest::blocking::get("https://httpbin.org/ip")?.json()?;
    println!("{:#?}", resp);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse args
    println!("[ ] Start");
    let args = Args::parse();

    // Open JSON config file
    let file = File::open(args.config).expect("Can't open config file");
    let data: Value = serde_json::from_reader(BufReader::new(file))?;

    // Do stuff
    do_stuff(&data)?;

    println!("[ ] End");
    Ok(())
}
