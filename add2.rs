// add2.rs
// add1 skipped as that intentionally introduces an error by trying to modify an immutable variable
// this is an introduction to a mutable variable

fn main() {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}