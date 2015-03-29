fn main() {
    let a = [1, 2, 3];

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e)
    }

    println!("");
    let mut v = vec![0; 10]; // ten zeroes
    v.push(10);
    for e in v.iter() {
        println!("{}", e)
    }

    println!("");
    let slice = &v[8..11];
    for e in slice.iter() {
        println!("{}", e)
    }
}
