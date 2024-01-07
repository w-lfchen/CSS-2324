use std::io;

fn main() {
    let (mut a, mut b);
    // get a and b
    while {
        println!("Please enter the first number (a) and press enter.");
        while {
            a = read_i32();
            a <= 0
        } {
            println!("a: {a} needs to be greater than 0!");
        }
        // a is now valid
        println!("Please enter the second number (b) and press enter.");
        while {
            b = read_i32();
            b <= 0
        } {
            println!("b : {b} needs to be greater than 0!");
        }
        // b is now valid, now last checks
        a <= b
    } {
        println!("a: {a} needs to be greater than b: {b}!")
    }
    // numbers are good, calculate now
    println!("Calculation of ggT({a}, {b}):");
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
    let tuple = if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x, y) = extended_euclidian_algorithm(b, a % b);
        (d, y, x - (a / b) * y)
    };
    println!("d: {}, x: {}, y: {}", tuple.0, tuple.1, tuple.2);
    tuple
}

// 3
// 3a
fn encrypt_ecb() -> String { // maybe change return type?
    let message = "Das CSS Team wuenscht Ihnen einen guten Rutsch ins neue Jahr! Wir freuen uns Sie in 2024 wieder zu sehen.";
    let key = "0000000"; // TODO: add correct number
    // iterate over blocks
    // iterate through blocks, encrypt
    todo!();
    return String::from("");
}

// 3b
fn encrypt_cbc() {
    todo!();
}

// 3c
fn encrypt_ctr() {
    todo!();
}
