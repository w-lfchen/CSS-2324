use std::io;

fn main() {
    println!("Please enter the first number and press enter.");
    let a = read_u32();
    println!("Please enter the second number and press enter.");
    let b = read_u32();

    extended_euclidian_algorithm(a, b);
}

fn read_u32() -> u32 {
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
        val = input.parse::<u32>();
        val.is_err()
    } {
        println!("Invalid input: <{input}>, please try again.");
        buffer.clear();
    }
    // invariant: val is now a valid u32
    val.expect("How did we get here?")
}

fn extended_euclidian_algorithm(a: u32, b: u32) {
    // testing implementation
    println!("a: {a}, b: {b}");
}
