use std::process;
//use std::error::Error;

pub fn parse(args: Vec<String>) -> Vec<(String, String)> {

    let f_format = ["f".to_string(), "-f".to_string(), "--float".to_string()];
    let i_format = ["i".to_string(), "-i".to_string(), "--int".to_string()];
    let h_format = ["h".to_string(), "-h".to_string(), "--high".to_string()];
    let l_format = ["l".to_string(), "-l".to_string(), "--low".to_string()];

    // this vector will store the parsed inputs
    let mut result_vec: Vec<(String, String)> = Vec::new();

    if f_format.contains(&args[1]) {
        result_vec.push((String::from("f"), String::from("")));
    } else if i_format.contains(&args[1]) {
        result_vec.push((String::from("i"), String::from("")));
    } else {
        println!("first argument must be -i (--int) or -f (--float)");
        process::exit(1);
    }

    for item in args.iter() {
        
        // This Code is under maintenance 
        // Once Done will prevent anyone from specifing deplicate -f or -i 

        /*
                if operation == String::from("-i") {
                    if *item == String::from("-f") {
                        println!("first argument i used ? cannot use f/i after that.");
                        process::exit(1);
                    }
                }
                if operation == String::from("-f") {
                    if *item == String::from("-i") {
                        println!("first argument f used ? cannot use i/f after that.");
                        process::exit(1);
                    }
                }
        */

        if l_format.contains(&item) {
            let position = match args.iter().position(|x| l_format.contains(x)) {
                Some(num) => num,
                None => {
                    println!("-l not found");
                    process::exit(1);
                }
            };

            let lower_value: i32 = match &args[position + 1].parse() {
                Ok(num) => {
                    //result_vec.push(*num);
                    *num
                }
                Err(_e) => {
                    println!("Expected a number after -l");
                    process::exit(1);
                }
            };
            let mut check = false;

            for i in args.iter() {
                if h_format.contains(i) {
                    check = true;
                }
            }

            if check == false {
                println!("for option -l specified -h is necessary");
                process::exit(1);
            }
            result_vec.push((String::from("l"), String::from(format!("{}", lower_value))));
        }
        if h_format.contains(&item) {
            let position = match args.iter().position(|x| h_format.contains(x)) {
                Some(num) => num,
                None => {
                    println!("-l not found");
                    process::exit(1);
                }
            };

            let higher_value: i32 = match &args[position + 1].parse() {
                Ok(num) => {
                    //result_vec.push(*num);
                    *num
                }
                Err(_e) => {
                    println!("Expected a number after -h");
                    process::exit(1);
                }
            };

    let mut not_present : bool = false;

            for (op, _) in result_vec.iter() {
                if l_format.contains(&op.to_string()) {
                    println!("got hereee");
                    not_present = true;
                }
            }

            if not_present {
                result_vec.push((String::from("l"), String::from("0")));
            }

            result_vec.push((String::from("h"), String::from(format!("{}", higher_value))));
        }
    }
    result_vec
}
