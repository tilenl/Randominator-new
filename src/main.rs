extern crate clap;
extern crate toml;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

fn main() -> std::io::Result<()>{
  // checking current directory
  let curr_dir = std::env::current_dir()?;
  println!("Current dir: {}", curr_dir.display());

  let file_name = "src/data.toml";

  let mut input_file = match File::open(Path::new(file_name)) {
    Ok(file) => file,
    Err(err) => {
      println!("File not found: {} ...\n... ERROR: {}", file_name, err);
      std::process::exit(1);
    }
  };

  let mut data_string = String::new();

  let read_bytes = input_file.read_to_string(&mut data_string)?;

  let data = match data_string.parse::<Value>() {
    Ok(value) => value,
    Err(err) => {
      println!("Toml can't parse data from {} ...\n ... TOML ERROR: {}", file_name, err);
      std::process::exit(1);
    }
  };
  
  println!("{:?}", data);

  Ok(())
}