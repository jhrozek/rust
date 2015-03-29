fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let val = 5;
    let y = add_one(val);
    println!("add_one(val) = {}\n", y);
}
