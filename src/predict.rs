use std::io;
use std::io::Read;
use std::fs::File;

fn read_file(mut file: std::fs::File, mileage: i32) -> (){
    let mut input = String::new();

    file.read_to_string(&mut input).expect("Cannot read file");
    let vec = input.split(" ").collect::<Vec<&str>>();
    let theta0: i32 = vec[0].trim().parse().expect("Theta0 is not a number");
    let theta1: i32 = vec[1].trim().parse().expect("Theta1 is not a number");
    println!("Price: {}", theta0 + (theta1 * mileage));
}

fn main() {
    let mut input = String::new();

    println!("Please enter a mileage:");
    io::stdin().read_line(&mut input).expect("Error on read");
    let mileage: i32 = input.trim().parse().expect("Not a Number");

    match File::open("/tmp/frale-co/data.txt") {
       Ok(res) => {read_file(res, mileage)},
        Err(_) => {println!("Price: 0")},
    };
}
