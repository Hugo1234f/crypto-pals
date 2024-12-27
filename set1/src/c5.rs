use std::io;

pub fn rep_key_xor() {
    println!("--------------------");
    println!("Implement repeating-key XOR");

    let mut key_s = String::new();
    let mut plaintext_s = String::new();

    println!("Enter key:");
    io::stdin()
        .read_line(&mut key_s)
        .expect("ERR: could not read key");
    //println("Key len: {}", key_s.len())

    println!("\nEnter plaintext:");
    io::stdin()
        .read_line(&mut plaintext_s)
        .expect("ERR: could not read plaintext");

    let key: Vec<u8> = key_s.trim().bytes().collect();
    let plaintext: Vec<u8> = plaintext_s.trim().bytes().collect();

    let hexed = rep_xor(&key, &plaintext);

    println!("Answer:");
    for byte in 0..plaintext.len() {
        print!("{}", format!("{:02x}", hexed[byte]));
    }
    println!("");
    
}

pub fn rep_xor(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u8> {
    let mut hexed: Vec<u8> = Vec::new();
    let mut index = 0;
    for c in plaintext {
        hexed.push(*c ^ key[index % key.len()]);

        index += 1;
    }

    hexed
}