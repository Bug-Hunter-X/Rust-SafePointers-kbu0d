fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe way to modify the first element
    println!("{:?}", v); //prints the vector v
}