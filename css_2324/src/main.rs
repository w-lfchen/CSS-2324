use std::io;

fn main() {
    println!("Please enter the first number and press enter.");
    let a = read_i32();
    println!("Please enter the second number and press enter.");
    let b = read_i32();

    let (d, k, l) = extended_euclidian_algorithm(a, b);
    println!("ggT({a}, {b}) = {d} = {k} * {a} + {l} * {b}");
}

fn read_i32() -> i32 {
    let mut buffer = String::new();
    let mut input;
    let mut val;
    while {
        // get input
        io::stdin()
            .read_line(&mut buffer)
            .expect("Unable to read from stdin!");
        input = buffer.trim();
        // parse input, evaluate block to val for the wile let loop
        val = input.parse::<i32>();
        val.is_err()
    } {
        println!("Invalid input: <{input}>, please try again.");
        buffer.clear();
    }
    // invariant: val is now a valid i32
    val.expect("How did we get here?")
}

fn extended_euclidian_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    println!("a: {a}, b: {b}");
    if b == 0 {
        println!("d: {a}, x: 1, y: 0");
        (a, 1, 0)
    } else {
        let (d, x, y) = extended_euclidian_algorithm(b, a % b);
        println!("d: {d}, x: {y}, y: {}", (x - (a / b) * y));
        (d, y, x - (a / b) * y)
    }
}
