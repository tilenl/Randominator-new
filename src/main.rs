extern crate clap;
extern crate rand;
extern crate toml;

use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

fn main() -> std::io::Result<()> {
  // checking current directory (for debuging purposes)
  let curr_dir = std::env::current_dir()?;
  println!("Current dir: {}", curr_dir.display());

  let file_name = "src/game_data.toml";

  let mut input_file = match File::open(Path::new(file_name)) {
    Ok(file) => file,
    Err(err) => {
      println!(
        "File not found: {} ...\n... ERROR: {}\nExiting ...\n",
        file_name, err
      );
      std::process::exit(1);
    }
  };

  let mut data_string = String::new();

  let _read_bytes = input_file.read_to_string(&mut data_string)?;

  let data = match data_string.parse::<Value>() {
    Ok(value) => value,
    Err(err) => {
      println!(
        "Toml can't parse data from {} ...\n ... TOML ERROR: {}\nExiting ...\n",
        file_name, err
      );
      std::process::exit(1);
    }
  };

  println!("RNG entry: {}", select_entry_from(&data, "data.action.action"));

  Ok(())
}

//TODO vrni Result, in v select_entry_form povej, da v data.human.bla bla ni field, da se bo vedlo kje ni ta field, ker morda je v celotnem datasetu... tukaj imamo premalo informacij, da bi to vse napisali
// Sprejme &str ali int in vrne kar je na temu indexu
fn get_index<'a>(data: &'a Value, index: &'a str) -> &'a Value {
  match data.get(index) {
    Some(dat) => dat,
    None => {
      // ne morem sprintat indexa, ker nima Debug traita implementiranega
      println!("ERROR: \"{}\" is not a proper field in dataset! Exiting ...\n", index);
      std::process::exit(1);
    }
  }
}

// Crawls to the entry table/array and pics a random entry
// IF the entry is not a final array or int..., it randomly crawls to that point
fn select_entry_from(data: &Value, entry: &str) -> String {
  let mut random_gen = rand::thread_rng();

  // value, from which we will generate a random entry
  let mut starting_data: &Value = data;

  // entry is a nested subtable ... needs parsing
  if entry.contains(".") {
    // razrezi in postopno pridobivaj entrije
    let slices: Vec<&str> = entry.trim().split(".").collect();
    println!("Slices: {:?}\n", slices);
    for ety in slices {
      starting_data = get_index(starting_data, ety);
    }
  } else {
    // drugace pa ne delamo slicov... in samo dobimo entry
    // lahko bi brez contains, ampak je tole bolj optimalno
    starting_data = get_index(starting_data, entry);
  }

  //println!("Starting_data: {:?}\n", starting_data);

  // select a random entry from the starting point data
  select_entry_from_rec(starting_data, &mut random_gen)
}

fn select_entry_from_rec(data: &Value, rng: &mut ThreadRng) -> String {
  if data.is_integer() {
    match data.as_integer() {
      Some(i) => i.to_string(),
      None => "Integer MISSING".to_string(),
    }
  } else if data.is_float() {
    match data.as_float() {
      Some(f) => f.to_string(),
      None => "float MISSING".to_string(),
    }
  } else if data.is_bool() {
    match data.as_bool() {
      Some(b) => b.to_string(),
      None => "Boolean MISSING".to_string(),
    }
  } else if data.is_datetime() {
    match data.as_datetime() {
      Some(dt) => dt.to_string(),
      None => "DateTime MISSING".to_string(),
    }
  } else if data.is_str() {
    match data.as_str() {
      Some(s) => s.to_string(),
      None => "String MISSING".to_string(),
    }
  } else if data.is_array() {
    match data.as_array() {
      Some(a) => {
        // pick a random array entry and return the string representation of it

        if a.len() == 0 {
          // if there is nothing to select, return nothing
          return "".to_string()
        }

        let random_entry_index = rng.gen_range(0..a.len());

        println!("Gen [{} -> {}] = {}\n",0, a.len()-1, random_entry_index);
        //? NOT using get_index, as we know this cannot fail
        let new_entry = data.get(random_entry_index).expect(format!("Cannot get {} from dataset", random_entry_index).as_str());

        select_entry_from_rec(&new_entry, rng)
      }
      None => "Array MISSING".to_string(),
    }
  } else {
    // else the entry is a table
    match data.as_table() {
      Some(t) => {
        // pick a random entry from tabel and return the value representation of it
        " ".to_string()
      }
      None => "Integer MISSING".to_string(),
    }
  }
}
