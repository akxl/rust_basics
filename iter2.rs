// iter3.rs

fn main() {

    // need to convert array into an Iterable
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

}