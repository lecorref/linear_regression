extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Training program")
        .version("1.0")
        .arg(Arg::with_name("DATASET")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .get_matches();
    println!("{:?}", String::from(matches.value_of("DATASET").unwrap()));

}
