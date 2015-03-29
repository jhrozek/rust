struct Point {
    x:  i32,
    y:  i32
}

struct Inches(i32);

fn main() {
    let origin = Point { x: 0, y: 0 };

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("integer_length is {}", integer_length);
}
