#[macro_use]
extern crate serde_derive;

extern crate csv;
extern crate serde;

use std::io;
use std::env;
use std::process;
use std::error::Error;
use std::ffi::OsString;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Voter {
    // we'll store lat/long as x/y for simplicity's sake
    zip: String,
    x: f64,
    y: f64,
}
fn run() -> Result<(), Box<Error>> {
    let mut voters = Vec::new();


//    let mut reader = csv::Reader::from_path(file_path);
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        let voter: Voter = result?;
        voters.push(voter);
    }
    Ok(())
}

#[allow(dead_code)]
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1){
        None => Err(From::from("expected 1 argument, got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}