use std::fs::{self, File}; 
use std::io::{self, prelude::*, BufReader}; 
struct Car{
    car_id: String,
    car_model: String,
    car_quantity: u32, 
    car_price: f64,

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let file = File::open("data.txt")?; 
    let reader = BufReader::new(file); 
    let mut cars: [Car; 18]; 

    for line in reader.lines(){
        let l = line.unwrap(); 
        let linea = l.split_ascii_whitespace(); 
        for w in linea {
            println!("{}", w); 
        }
    }
    Ok(())
}