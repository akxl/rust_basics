// slice2.rs
// Accessing out-of-bounds memory and the Optional

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first); // prints Some(1)
    println!("last {:?}", last); // prints None

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    // long-winded way
    let safe_last = if last.is_some() {
        *last.unwrap() // the value in last is &i32 so will need to dereference it
    } else {
        -1
    };
    println!("safe_last {}", safe_last);

    // shorter way
    let safe_last_short = *slice.get(5).unwrap_or(&-1);
    println!("safe_last_short {}", safe_last_short);
}