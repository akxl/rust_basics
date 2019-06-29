// vec3.rs
// use the macro vec! to initialize a vector.
// remove values from the end with .pop()
// use .extend(i: Iterator) to extend vector.

fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);


    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]); // vectors can compare with other vectors AND slices
    // use .insert() and .remove() to add/delete at arbitrary locations
    

}