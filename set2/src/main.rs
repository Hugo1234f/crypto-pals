pub mod c9;

use std::io;

fn main() {
    println!("--------------------");
    println!("Lab1 - Cryptopals:");
    'mainLoop: loop {
        let result = menu();
        match result {
            0 => break 'mainLoop,
            1 => c9::pkcs7(),
            _ => continue,
        }
    }
}

//Handle inputs
fn menu() -> u32 {
    println!("--------------------");
    println!("0. Exit");
    println!("9. Implement PKCS#7 padding"); 

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

