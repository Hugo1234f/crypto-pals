pub mod c1;
pub mod c2;
pub mod c3;
pub mod c4;
pub mod c5;
pub mod c6;
pub mod c7;
pub mod c8;

use std::io;


fn main() {
    println!("--------------------");
    println!("Lab1 - Cryptopals:");
    'mainLoop: loop {
        let result = menu();
        match result {
            0 => break 'mainLoop,
            1 => c1::hex_bo_base64(),
            2 => c2::xor(),
            3 => c3::sb_xor(),
            4 => c4::single_character_xor(),
            5 => c5::rep_key_xor(),
            6 => c6::break_rep_key_xor(),
            7 => c7::aes_in_ecb_mode(),
            8 => c8::detect_ecb(),
            _ => continue,
        }
    }
}

//Handle inputs
fn menu() -> u32 {
    println!("--------------------");
    println!("0. Exit");
    println!("1. Convert Hex to Base64");
    println!("2. Fixed XOR");
    println!("3. Single-byte XOR cipher");
    println!("4. Detect single-character XOR");
    println!("5. Implement repeating-key XOR");
    println!("6. Break repeating-key XOR");
    println!("7. AES in ECB mode");
    println!("8. Detect AES in ECB mode");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERR: Could not read input");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_num) => 99,
    };
        
    input
}

