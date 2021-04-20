extern crate clap;
extern crate rand;
extern crate toml;

use clap::App;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

fn main() -> std::io::Result<()> {
  let matches = App::new("Randominator")
    .version("3.0")
    .about("Generate random sentences based on given data and templates")
    .author("Tilen Lampret")
    .arg(
      clap::Arg::with_name("numOfSent")
        .short("n")
        .long("numOfSentences")
        .number_of_values(1)
        .takes_value(true)
        .value_name("INTEGER")
        .default_value("1")
        .help("Number of generated sentences"),
    )
    .arg(
      clap::Arg::with_name("dataset")
        .short("d")
        .long("dataset")
        .index(1)
        .number_of_values(1)
        .takes_value(true)
        .required(true)
        .value_name("FILE")
        .help("TOML file, that includes [data] and [templates] tables"),
    )
    .arg(
      clap::Arg::with_name("template")
        .short("t")
        .long("template")
        .number_of_values(1)
        .value_name("STRING")
        .takes_value(true)
        .help("Name of template (in dataset file) that will be used"),
    )
    .get_matches();

  let file_name = matches.value_of("dataset").unwrap();
  let mut input_file = match File::open(Path::new(file_name)) {
    Ok(file) => file,
    Err(err) => {
      let curr_dir = std::env::current_dir()?;
      eprintln!(
        "File \"{}\" not found in current directory \"{}\"\nERROR: {}\nExiting ...",
        file_name,
        curr_dir.display(),
        err
      );
      std::process::exit(1);
    }
  };

  let mut data_string = String::new();
  let _read_bytes = input_file.read_to_string(&mut data_string)?;

  let file = match data_string.parse::<Value>() {
    Ok(value) => value,
    Err(err) => {
      eprintln!(
        "Toml can't parse data from {}\nTOML ERROR: {}",
        file_name, err
      );
      std::process::exit(1);
    }
  };

  // Extract templates and data from toml file
  let templates = match file.get("templates") {
    Some(templ) => templ,
    None => {
      eprintln!(
        "ERROR: No definition of [templates] exists in file {}",
        file_name
      );
      std::process::exit(1);
    }
  };

  let data = match file.get("data") {
    Some(dat) => dat,
    None => {
      eprintln!(
        "ERROR: No definition of [data] exists in file {}",
        file_name
      );
      std::process::exit(1);
    }
  };

  // DATA AND TEMPLATES LOADED SUCCESSFULLY

  let mut random_gen = rand::thread_rng();

  // we unwrap it, because default value that will be passed is 1 ... we will ALWAYS get a value (maybe not valid number)
  let num_of_sent: &str = matches.value_of("numOfSent").unwrap();
  let num_of_sent: usize = match num_of_sent.parse::<usize>() {
    Ok(num) => {
      if num < 1 {
        eprintln!(
          "WARNING: number of generated sentences {} is lower than 1... generating 1 sentence",
          num
        );
        1
      } else {
        num
      }
    }
    Err(e) => {
      eprintln!(
        "ERROR: number of sentences: \"{}\", in not a valid number\n{}",
        num_of_sent, e
      );
      std::process::exit(1);
    }
  };

  let template = match matches.value_of("template") {
    Some(temp) => match select_entry_from(&templates, temp, &mut random_gen) {
      Ok(t) => t,
      Err(e) => {
        eprintln!(
          "ERROR: template \"{}\" is not specified in [templates] table in file \"{}\"\n{}",
          temp, file_name, e
        );
        std::process::exit(1);
      }
    },
    None => {
      let rand_template = match select_entry_from(&file, "templates", &mut random_gen) {
        Ok(t) => t,
        Err(e) => {
          eprintln!(
            "ERROR: Could not get a random template from [templates] in file {}\n{}",
            file_name, e
          );
          std::process::exit(1);
        }
      };
      eprintln!(
        "WARNING: No template was provided... using \"{}\" template from dataset",
        rand_template
      );
      rand_template
    }
  };

  //println!("{} from {} with template: {}", num_of_sent, file_name, template);

  for _i in 0..num_of_sent {
    match gen_with(&data, &template, &mut random_gen) {
      Ok(gen) => println!("{}", gen),
      Err(e) => {
        eprintln!(
          "ERROR: Could not generate template {} from data\n{}",
          template, e
        );
        std::process::exit(1);
      }
    }
  }

  Ok(())
}

// Crawls to the entry table/array and pics a random entry
// IF the entry is not a final array or int..., it randomly crawls to that point
fn select_entry_from<'a>(
  data: &'a Value,
  entry: &str,
  rng: &mut ThreadRng,
) -> Result<String, String> {
  // value, from which we will generate a random entry
  let mut starting_data: &Value = data;

  // razrezi in postopno pridobivaj entries
  let slices: Vec<&str> = entry.trim().split(".").collect();
  //println!("Slices: {:?}\n", slices);
  for ety in slices {
    starting_data = match starting_data.get(ety) {
      Some(d) => d,
      None => {
        return Err(format!(
          "ERROR: \"{}\" is not a proper field in {}",
          ety, entry
        ))
      }
    };
  }

  //println!("Starting_data: {:?}\n", starting_data);

  // select a random entry from the starting point data
  select_entry_from_rec(starting_data, rng)
}

fn select_entry_from_rec<'a>(data: &'a Value, rng: &mut ThreadRng) -> Result<String, String> {
  if data.is_integer() {
    match data.as_integer() {
      Some(i) => Ok(i.to_string()),
      None => Err("Integer MISSING".to_string()),
    }
  } else if data.is_float() {
    match data.as_float() {
      Some(f) => Ok(f.to_string()),
      None => Err("float MISSING".to_string()),
    }
  } else if data.is_bool() {
    match data.as_bool() {
      Some(b) => Ok(b.to_string()),
      None => Err("Boolean MISSING".to_string()),
    }
  } else if data.is_datetime() {
    match data.as_datetime() {
      Some(dt) => Ok(dt.to_string()),
      None => Err("DateTime MISSING".to_string()),
    }
  } else if data.is_str() {
    match data.as_str() {
      Some(s) => Ok(s.to_string()),
      None => Err("String MISSING".to_string()),
    }
  } else if data.is_array() {
    match data.as_array() {
      Some(a) => {
        // pick a random array entry and return the string representation of it

        if a.len() == 0 {
          // if there is nothing to select, return nothing
          return Ok("".to_string());
        }

        // select a random entry index from range [0, 1, 2, ..., a.len() - 1]
        let random_entry_index = rng.gen_range(0..a.len());

        //println!("Gen [{} -> {}] = {}\n", 0, a.len() - 1, random_entry_index);
        // tables are indexed by string, so we get the key of generated index

        //I guess this cannot fail, as we already bug proof and return "" before this statement
        let new_entry = match data.get(random_entry_index) {
          Some(dat) => dat,
          None => return Err(format!("Cannot get {} from dataset", random_entry_index)),
        };

        select_entry_from_rec(new_entry, rng)
      }
      None => Err("Array MISSING".to_string()),
    }
  } else {
    // else the entry is a table
    match data.as_table() {
      Some(t) => {
        // pick a random subvalue and return the string representation of it
        let keys = t.keys();
        //println!("Num of keys: {}", keys.len());
        // if there are no subvalues, return nothing
        if keys.len() < 1 {
          return Ok("".to_string());
        }

        // select a random entry index from range [0, 1, 2, ..., keys.len() - 1]
        let random_entry_index = rng.gen_range(0..keys.len());
        let mut random_entry_str = &String::from(" ");
        for (i, e) in keys.enumerate() {
          if i == random_entry_index {
            random_entry_str = e;
            break;
          }
        }
        //println!("Chose {}\n", random_entry_str);
        //I guess this cannot fail, as we already bug proof and return "" before this statement
        let new_entry = match data.get(random_entry_str) {
          Some(dt) => dt,
          None => return Err(format!("Cannot get {} from dataset", random_entry_index)),
        };

        // pick a random entry from tabel and return the value representation of it
        select_entry_from_rec(new_entry, rng)
      }
      None => Err("Integer MISSING".to_string()),
    }
  }
}

// fn select_random<'a>(data: &'a Value, rng: &mut ThreadRng) -> &'a Value {

// }

fn gen_with<'a>(data: &'a Value, template: &str, rng: &mut ThreadRng) -> Result<String, String> {
  let mut slices: Vec<String> = template.split(|c| c == '<' || c == '>').map(|s| s.to_string()).collect();

  for slice in slices.iter_mut() {
    // we pass the slice without the first char, which is '?' or '!'
    if slice.starts_with('!') {
      let rand_entry = select_entry_from(data, &slice[1..], rng)?;
      *slice = rand_entry;
    } else if slice.starts_with('?') {
      if rng.gen_range(0..=1) == 1 {
        let rand_entry = select_entry_from(data, &slice[1..], rng)?;
        *slice = rand_entry;
      } else {
        *slice = "".to_string();
      }
    }
  }

  Ok(slices.concat().replace("  ", " "))
}
