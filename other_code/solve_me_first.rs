use std::io;

fn main() {

    let mut _number_string_1 = String::new();
    let mut _number_string_2 = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut _number_string_1)
        .ok()
        .expect("read error");
    
    stdin.read_line(&mut _number_string_2)
        .ok()
        .expect("read error");
    
    let _number_1 = _number_string_1
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .expect("parse error");
    let _number_2 = _number_string_2
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .expect("parse error");
    
    println!("{}", _number_1 + _number_2);

}