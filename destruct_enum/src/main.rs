enum OptionalInt {
    Value(i32),
    Missing,
}

fn main() {
    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("{}", n),
        OptionalInt::Missing => println!("Missing"),
    }

    match y {
        OptionalInt::Value(n) => println!("{}", n),
        OptionalInt::Missing => println!("Missing"),
    }
}
