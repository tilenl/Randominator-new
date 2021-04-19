extern crate clap;
extern crate toml;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

fn main() -> std::io::Result<()>{
  // checking current directory (for debuging purposes)
  let curr_dir = std::env::current_dir()?;
  println!("Current dir: {}", curr_dir.display());

  let file_name = "src/names_data.toml";

  let mut input_file = match File::open(Path::new(file_name)) {
    Ok(file) => file,
    Err(err) => {
      println!("File not found: {} ...\n... ERROR: {}\nExiting ...\n", file_name, err);
      std::process::exit(1);
    }
  };

  let mut data_string = String::new();

  let read_bytes = input_file.read_to_string(&mut data_string)?;

  let data = match data_string.parse::<Value>() {
    Ok(value) => value,
    Err(err) => {
      println!("Toml can't parse data from {} ...\n ... TOML ERROR: {}\nExiting ...\n", file_name, err);
      std::process::exit(1);
    }
  };
  println!("{:?}", get_index(&data, "template"));

  println!("Templates: \n{}", traverse_data_tree(&data["templates"]));
  println!("Data: \n{}", traverse_data_tree(&data["data"]));

  Ok(())
}

fn get_index<'a>(data: &'a Value, index: &str) -> &'a Value {
  match data.get(index) {
    Some(dat) => dat,
    None => {
      println!("ERROR: {} is not a field in dataset! Exiting ...\n", index);
      std::process::exit(1);
    }
  }
}

fn traverse_data_tree(data: &Value) -> String {
  String::new()
}