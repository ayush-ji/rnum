use rnum;
use std::env;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("not enough arguments");
        process::exit(1);
    }
    rnum::run(args);
    //println!("{:?}", args);
}
