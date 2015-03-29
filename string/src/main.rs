fn print_slice(slice: &str) {
    println!("Got: {}", slice)
}


fn main() {
    let string = "Hello".to_string();
    print_slice(&string);
}
