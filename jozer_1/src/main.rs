use std::fs::{self, File}; 
use std::io::{self, prelude::*, BufReader}; 
struct Car{
    car_id: String,
    car_model: String,
    car_quantity: u32, 
    car_price: f64,

}

fn main() {
    println!("Hello, world!");
    let file = File::open("data.txt"); 
    let buf_reader = BufReader::new(file); 
    
    for lines in buf_reader.lines(){
        println!("{}", line?); 
    }
    Ok(()); 
}
