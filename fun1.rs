// fun1.rs
// writing a user-defined function

fn sqr(x: f64) -> f64 {
    return x * x;
}

// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
    println!("Abs of -45.7 is {}", abs(-45.7));
    println!("clamp 9.99 between 5.0 and 8.0 is {}", clamp(9.99, 5.0, 8.0));
    println!("factorial of 10 is {}", factorial(10))

}