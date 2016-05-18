extern crate clap;
extern crate csv;
use clap::{Arg, App};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
type Row = (f64, f64);

fn create_file(t0: f64, t1: f64, min: f64, max: f64) -> () {
    let path = Path::new("/tmp/frale-co/data.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(format!("{} {} {} {}", t0, t1, min, max).as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
fn main() {
    let matches = App::new("Training program")
        .version("1.0")
        .arg(Arg::with_name("DATASET")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .get_matches();
    let mut rdr = csv::Reader::from_file(String::from(matches.value_of("DATASET").unwrap())).unwrap();
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let &(maxx, _) = rows.iter().max_by_key(|&&(x, _)| x as i32).unwrap();
    let &(minx, _) = rows.iter().min_by_key(|&&(x, _)| x as i32).unwrap();

    let scaledrows = rows.iter().map(|&(x, y)| ((x - minx) / (maxx - minx), y)).collect::<Vec<Row>>();
    let len: f64 = rows.len() as f64;
    let learning_rate: f64 = 0.1;

    let mut theta0: f64 = 0.0;
    let mut theta1: f64 = 0.0;

    for _ in 1..10000 {
        let (test, test1) = scaledrows.iter().fold((0.0, 0.0), |(x, y), &(km, price)|{
            let theta = ((theta0 + theta1 * km) - price) / len;
            (x + theta, y + theta * km)
        }
        );
        theta0 -= test * learning_rate;
        theta1 -= test1 * learning_rate;
    }
    create_file(theta0, theta1, minx, maxx);
}
