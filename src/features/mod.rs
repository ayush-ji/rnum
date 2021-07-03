extern crate rand;
use rand::prelude::*;

pub fn handle_float(high : i32, low : i32) {
    let mut rng = rand::thread_rng();
    let y: f64 = (rng.gen::<f64>() * (high-low) as f64) + low as f64;
    println!("float:: {}", y);
}

pub fn handle_int(high : i32, low : i32) {
    let mut rng = rand::thread_rng();
    let y: f64 = (rng.gen::<f64>() * (high-low) as f64) + low as f64;
    println!("int ::{}", y as i64);
}