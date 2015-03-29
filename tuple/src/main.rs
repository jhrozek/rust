fn next_two(x: i32) -> (i32, i32)
{
    (x+1, x+2)
}

fn main() {
    let tup = (1, 2, 3);
    let tup2 = (1, 2, 3);
    let tup3 = (2, 3, 4);

    let hetero_tup = (4, "hello");

    let (x, y, z) = tup;

    println!("x y z are {} {} {}", x, y, z);

    if tup == tup2 {
        println!("tup equals tup2");
    }

    if tup2 == tup3 {
        println!("tup2 equals tup3");
    }

    let (n1, n2) = next_two(5);

    println!("next_two of 5 is {}, {}", n1, n2);
}
