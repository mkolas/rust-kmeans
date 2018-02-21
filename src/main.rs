extern crate csv;

use std::io;

struct Voter {
    // we'll store lat/long as x/y for simplicity's sake
    zip: String,
    x: f64,
    y: f64
}
fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut reader = csv::Reader::from_path(file_path);
    for result in reader.records() {
        let record = result?;
        let zip = &record[0];
        let y: f64 = record[1].parse()?;
        let x: f64 = record[2].parse()?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1){
        None => Err(From::from("expected 1 argument, got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit();
    }
}