extern crate csv;

use std::env;
use std::process;
use std::error::Error;
use std::ffi::OsString;


#[derive(Debug, Default)]
struct Voter {
    // we'll store lat/long as x/y for simplicity's sake
    zip: String,
    x: f64,
    y: f64,
    primary: i32,
    secondary: i32,
    distances: Vec<f64>
}


fn run() -> Result<(), Box<Error>> {
    let k = 14;
    let mut voters = Vec::new();
    let file_path = get_first_arg()?;

    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        let zip = String::from(&record[0]);
        let x: f64 = record[2].parse().unwrap();
        let y: f64 = record[1].parse().unwrap();
        let distances = vec![std::f64::INFINITY; k];
        let mut voter = Voter{zip, x, y, distances, ..Default::default() };
        println!("{:?}", voter);
        voters.push(voter);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1){
        None => Err(From::from("expected 1 argument, got none")),
        Some(file_path) => Ok(file_path),
    }
}

//fn get_positive_infinity_vector(k: i32) -> Vec<i32> {
//    vec![std::f64::INFINITY, k]
//}
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}