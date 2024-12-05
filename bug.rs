fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; //modify the first element
    }
    println!("{:?}", v); //print the vector v
}