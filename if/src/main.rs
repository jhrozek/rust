fn main() {
    let x = 5;

    if x == 4 {
        println!("X equals 4\n");
    } else if x == 5 {
        println!("X equals 5\n");
    } else {
        println!("X equals {}\n", x);
    }

    let y = if x == 4 { "four" } else { "NaN" };
    println!("Y is {}\n", y);
}
