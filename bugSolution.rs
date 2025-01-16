fn main() {    let mut v = vec![1, 2, 3];    v.clear(); // Safe way to clear the vector
    println!("v: {:?}", v); // v will now be an empty vector}
