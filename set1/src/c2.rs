use std::io;

pub fn xor() {
    println!("--------------------");
    println!("Fixed XOR:");
    println!("NOTE: Buffers must be of equal length!");
    println!("First buffer:");
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    
    

    io::stdin()
        .read_line(&mut buf1)
        .expect("ERR: Could not read input");
    
    println!("");
    println!("Second buffer:");

    io::stdin()
        .read_line(&mut buf2)
        .expect("ERR: Could not read input");
    println!("");

    let xored_buf = calc_xor(&buf1[..], &buf2[..]);
    println!("Xored result:");
    println!("{xored_buf}");
}
fn calc_xor(buf1: &str, buf2: &str) -> String {
    let mut _bytes1: Vec<u8> = Vec::new();
    let mut _bytes2: Vec<u8> = Vec::new();

    for c in 0..(buf1.len()/2) {
        let _b1 = u8::from_str_radix(&buf1[2*c..2*c+2], 16);
        let _b2 = u8::from_str_radix(&buf2[2*c..2*c+2], 16);

        match _b1 {
            Ok(e) => _bytes1.push(e),
            Err(_) => continue,
        };
        match _b2 {
            Ok(e) => _bytes2.push(e),
            Err(_) => continue,
        };
    }

    for c in 0..(_bytes1.len()) {
        _bytes1[c] ^= _bytes2[c];
    }

    String::from_utf8(_bytes1).unwrap()
}