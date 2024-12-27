use std::io;
use base64::{Engine as _, engine::general_purpose};

pub fn hex_bo_base64() {
    println!("--------------------");
    println!("Hex to Base64 conversion:");
    println!("Hex text: ");

    let mut plaintext = String::new();

    //Read plaintext from user
    io::stdin()
        .read_line(&mut plaintext)
        .expect("ERR: Could not read input");
    println!("");

    let b64 = calc_hex_to_base64(&plaintext[..]);
    println!("Base64 text:");
    println!("{b64}");
        
}
fn calc_hex_to_base64(hex_text: &str) -> String {
    let mut _bytes: Vec<u8> = Vec::new();
    //Converts the string representation (per byte) to decimal and adds it to the _bytes vector.
    for c in 0..(hex_text.len()/2) {
        let _byte = u8::from_str_radix(&hex_text[2*c..2*c+2], 16); //read every even to (even+1) (i.e. byte) chars
        match _byte {
            Ok(e) => _bytes.push(e),
            Err(_) => continue,
        };
    }

    //Convert and print full string
    let b64 = general_purpose::STANDARD.encode(&_bytes);
    b64
}