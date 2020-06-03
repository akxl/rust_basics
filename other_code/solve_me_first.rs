use std::io;

fn parse_string_to_number(input: String) -> i32 {
    return input.trim()
            .parse::<i32>()
            .ok()
            .expect("parse error");
}

fn read_string_from_stdin() -> String {
    let mut the_string = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut the_string)
        .ok()
        .expect("read error");
    return the_string;
}

fn main() {

    let _number_string_1 = read_string_from_stdin();
    let _number_string_2 = read_string_from_stdin();
    
    let _number_1 = parse_string_to_number(_number_string_1);
    let _number_2 = parse_string_to_number(_number_string_2);
    
    println!("{}", _number_1 + _number_2);


}