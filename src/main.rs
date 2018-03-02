extern crate csv;
extern crate rand;

use std::cmp::Ordering;
use std::env;
use std::process;
use std::error::Error;
use std::ffi::OsString;
use rand::distributions::{Sample, Range};


#[derive(Debug, Default)]
struct Voter {
    // we'll store lat/long as x/y for simplicity's sake
    zip: String,
    x: f64,
    y: f64,
    primary: usize,
    secondary: usize,
    distances: Vec<f64>
}

#[derive(Debug, Default)]
struct Center {
    x: f64,
    y: f64,
}

impl Voter {
    fn priority(&self) -> f64 {
        self.distances[self.secondary] - self.distances[self.primary]
    }

    fn gain(&self, i: usize) -> f64 {
        self.distances[self.primary] - self.distances[i]
    }
}

// Sorts by largest benefit of assigning to preferred to cluster
impl Ord for Voter {
    fn cmp(&self, other: &Voter) -> Ordering {
        self.priority().partial_cmp(&other.priority()).unwrap()
    }
}

impl PartialOrd for Voter {
    fn partial_cmp(&self, other: &Voter) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Voter {
    fn eq(&self, other: &Voter) -> bool {
        self.priority() == other.priority()
    }
}
impl Eq for Voter {}


fn calculate_dists(v: &Voter, centers:&Vec<Center>) -> Vec<f64> {
    let mut distances = Vec::new();
    for c in centers {
        distances.push(dist(v, c));
    }
    return distances;
}

fn dist(v: &Voter, c:&Center) -> f64 {
    ((c.x - v.x).powf(2.0) +(c.y-v.y).powf(2.0)).sqrt()
}

fn run() -> Result<(), Box<Error>> {
    let k = 14;
    let mut voters = Vec::new();
    let file_path = get_first_arg()?;

    // initialise clusters
    let mut clusters = Vec::new();
    let mut rng = rand::thread_rng();
    let mut x_between = Range::new(-90.,-82.);
    let mut y_between = Range::new(41., 48.);
    for _ in 0..k {
        let x = x_between.sample(&mut rng);
        let y = y_between.sample(&mut rng);
        clusters.push(Center {x, y});
    }
    println!("{:?}", clusters);


    // initialize voters
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        let zip = String::from(&record[0]);
        let x: f64 = record[2].parse().unwrap();
        let y: f64 = record[1].parse().unwrap();
        let mut voter = Voter { zip, x, y, ..Default::default() };
        voter.distances = calculate_dists(&voter, &clusters);
        for i in 0..k {
            if voter.distances[i] < voter.distances[voter.primary] {
                voter.primary = i;
            } else if voter.distances[i] > voter.distances[voter.secondary] {
                voter.secondary = i;
            }
        }
        voters.push(voter);
    }

    voters.sort_unstable();
    println!("{:?}", voters);

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
        process::exit(1);
    }
}