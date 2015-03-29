fn main() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true };
    }

    x = 9;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break };
    }

    for x in 0..10 {
        if x % 2 ==0 { continue };
        println!("{}", x);
    }
}
