use std::io;

fn main() {
    const MATR_NR: &str = "0000000"; // TODO: add correct number
    const SHA_256_HASH_5A: &str = ""; // TODO: use CSS ID, write csv parser?
    const SALT_5A: &str = "";
    const SHA_256_HASH_5B: &str = "";
    const SALT_5B: &str = "";

    // exercise 1
    let (a, b) = get_a_b();
    println!("Calculation of ggT({a}, {b}):");
    let (d, k, l) = extended_euclidian_algorithm(a, b);
    println!("ggT({a}, {b}) = {d} = {k} * {a} + {l} * {b}");

    // exercise 3
    const MESSAGE: &str = "Das CSS Team wuenscht Ihnen einen guten Rutsch ins neue Jahr! Wir freuen uns Sie in 2024 wieder zu sehen.";
    let _ = encrypt_ecb(MESSAGE, MATR_NR);
    let _ = encrypt_cbc(MESSAGE, MATR_NR);
    let _ = encrypt_ctr(MESSAGE, MATR_NR);

    // exercise 5
    const PASSWORDS_PATH: &str = "../rockyou-75.txt";
    match find_password_5a(SHA_256_HASH_5A, SALT_5A, PASSWORDS_PATH) {
        Ok(password) => println!("5a: Found password: <{password}>"),
        Err(e) => println!("5a: {}", e),
    }
    match find_password_5b(SHA_256_HASH_5B, SALT_5B, PASSWORDS_PATH) {
        Ok((password, iterations)) => {
            println!("5b: Found password: <{password}>; Number of iterations: {iterations}")
        }
        Err(e) => println!("5b: {}", e),
    }
}

fn find_password_5a(sha256_hash: &str, salt: &str, path: &str) -> Result<String, std::io::Error> {
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use std::fs::read_to_string;

    let mut hasher = Sha256::new();
    let string = read_to_string(path)?;
    let lines = string.lines();
    for password in lines {
        hasher.input_str(&(password.to_owned() + salt));
        let hashed_str = hasher.result_str();
        if hashed_str == sha256_hash {
            return Ok(password.to_owned());
        }
        hasher.reset();
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Unable to find password for hash: <{sha256_hash}> with salt: <{salt}>"),
    ))
}

fn find_password_5b(
    sha256_hash: &str,
    salt: &str,
    path: &str,
) -> Result<(String, i32), std::io::Error> {
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use std::fs::read_to_string;

    let mut hasher = Sha256::new();
    let string = read_to_string(path)?;
    let lines = string.lines();
    let mut buffer;
    for password in lines {
        buffer = password.to_owned() + salt;
        for i in 1..101 {
            hasher.input_str(&buffer);
            buffer = hasher.result_str();
            if buffer == sha256_hash {
                return Ok((password.to_owned(), i));
            }
            hasher.reset();
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Unable to find password for hash: <{sha256_hash}> with salt: <{salt}>"),
    ))
}

fn get_a_b() -> (i32, i32) {
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
    (a, b)
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

// exercise 3
// 3a
fn encrypt_ecb(message: &str, key: &str) -> String {
    let mut cipher_vec = Vec::<u8>::with_capacity(105);
    let message_bytes = message.as_bytes();
    let key_bytes = key.as_bytes();
    // iterate over blocks
    for i in 0..15 {
        // iterate through blocks, encrypt
        for j in 0..7 {
            let msg_byte = message_bytes[i * 7 + j];
            let key_byte = key_bytes[j];
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
    let message_bytes = message.as_bytes();
    let key_bytes = key.as_bytes();
    // set up initial vector in buffer
    let mut buffer = [0_u8; 7];
    let mut cipher_vec = Vec::<u8>::with_capacity(105);
    // iterate over blocks
    for i in 0..15 {
        // get block^buffer
        let mut tmp = Vec::<u8>::with_capacity(7);
        for j in 0..7 {
            let msg_byte = message_bytes[i * 7 + j];
            let buf_byte = buffer[j];
            tmp.push(msg_byte ^ buf_byte);
        }
        // encrypt to get encrypted block
        for j in 0..7 {
            let tmp_byte = tmp[j];
            let key_byte = key_bytes[j];
            buffer[j] = tmp_byte ^ key_byte;
        }
        // copy into cipher
        cipher_vec.extend(buffer);
    }
    std::str::from_utf8(&cipher_vec)
        .expect("3b: bytes to utf8 conversion failed!")
        .to_string()
}

// 3c
fn encrypt_ctr(message: &str, key: &str) -> String {
    let message_bytes = message.as_bytes();
    let key_bytes = key.as_bytes();
    // set up initial vector
    let iv = [0_u8; 7];
    let mut cipher_vec = Vec::<u8>::with_capacity(105);
    // iterate over blocks
    for i in 0..15 {
        // get iv value
        // TODO: this looks scuffed, improve this bit
        let mut tmp = iv;
        tmp[6] += i as u8;
        // encrypt
        for j in 0..7 {
            let tmp_byte = tmp[j];
            let key_byte = key_bytes[j];
            tmp[j] = tmp_byte ^ key_byte;
        }
        // xor the now encrypted value in tmp
        for j in 0..7 {
            let msg_byte = message_bytes[i * 7 + j];
            let tmp_byte = tmp[j];
            cipher_vec.push(msg_byte ^ tmp_byte);
        }
    }
    std::str::from_utf8(&cipher_vec)
        .expect("3c: bytes to utf8 conversion failed!")
        .to_string()
}
