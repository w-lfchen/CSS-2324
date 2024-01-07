use std::io;

fn main() {
    const MATR_NR: &str = "0000000"; // TODO: add correct number

    const MESSAGE: &str = "Das CSS Team wuenscht Ihnen einen guten Rutsch ins neue Jahr! Wir freuen uns Sie in 2024 wieder zu sehen.";
    let _ = encrypt_ecb(MESSAGE, MATR_NR);
    let _ = encrypt_cbc(MESSAGE, MATR_NR);
    let _ = encrypt_ctr(MESSAGE, MATR_NR);

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
fn encrypt_ecb(message: &str, key: &str) -> String {
    // maybe change return type?
    let mut cipher_vec = Vec::<u8>::new();
    // iterate over blocks
    for i in 0..15 {
        // iterate through blocks, encrypt
        for j in 0..7 {
            let msg_byte = message.as_bytes()[i * 7 + j];
            let key_byte = key.as_bytes()[j];
            cipher_vec.push(msg_byte ^ key_byte);
        }
    }
    // all numbers are in the vec, now convert to string and return
    std::str::from_utf8(&cipher_vec)
        .expect("3a: bytes to utf8 conversion failed!")
        .to_string()
}

// 3b
fn encrypt_cbc(message: &str, key: &str) -> String {
    let _ = message;
    let _ = key;
    todo!();
}

// 3c
fn encrypt_ctr(message: &str, key: &str) -> String {
    let _ = message;
    let _ = key;
    todo!();
}
