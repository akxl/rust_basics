// string1.rs
// relationship between String and &str (e.g. std::string vs const char* in c++)

fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly!";
    let s = text.to_string();

    dump(text);
    dump(&s); // coersing String into &str
}
