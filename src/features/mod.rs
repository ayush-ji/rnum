extern crate rand;
use rand::prelude::*;
use std::process;

fn check_high_low(high: i32, low: i32) {
    if high <= low {
        eprintln!("check your high low value");
        process::exit(1);
    }
}

pub fn handle_float(high: i32, low: i32) {
    check_high_low(high, low);
    println!("HIGH :: {} || LOW :: {}", high, low);
    let mut rng = rand::thread_rng();
    let y: f64 = (rng.gen::<f64>() * (high - low) as f64) + low as f64;
    println!("float:: {}", y);
}

pub fn handle_int(high: i32, low: i32) {
    check_high_low(high, low);
    println!("HIGH :: {} || LOW :: {}", high, low);
    let mut rng = rand::thread_rng();
    let y: f64 = (rng.gen::<f64>() * (high - low) as f64) + low as f64;
    println!("int ::{}", y as i64);
}
