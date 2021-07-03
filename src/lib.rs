pub mod test;
pub mod logic;
pub mod features;

pub fn run( args : Vec<String> ) {

/**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**/
/**/ let f_format = ["f".to_string(), "-f".to_string(), "--float".to_string()]; /**/
/**/ let i_format = ["i".to_string(), "-i".to_string(),   "--int".to_string()]; /**/
/**/ let h_format = ["h".to_string(), "-h".to_string(),  "--high".to_string()]; /**/
/**/ let l_format = ["l".to_string(), "-l".to_string(),   "--low".to_string()]; /**/
/**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**//**/

    let parsed = logic::parse(args);
    let mut high : i32  = 0 ;
    let mut low : i32  = 0;
    for (op, value) in parsed.iter() {
        if l_format.contains(op) {
            high = match value.parse::<i32>() {
                Ok(num) => num,
                Err(_e) => panic!("exit"),
            }
        } 

        if h_format.contains(op) {
            low = match value.parse::<i32>() {
                Ok(num) => num,
                Err(_e) => panic!("EXIT"),
            };
        }
    }

        if i_format.contains(&parsed[0].0) {
            features::handle_int(high, low);
        } 

        if f_format.contains(&parsed[0].0) {
            features::handle_float(high, low);
        }

    println!("{:?}", parsed);


}