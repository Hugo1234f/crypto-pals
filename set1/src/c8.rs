use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub fn detect_ecb() {
    println!("--------------------");
    println!("Detect AES in ECB mode");
    let ciphers: Vec<Vec<u8>> = get_file_content();
    detect(&ciphers);
}

pub fn get_file_content() -> Vec<Vec<u8>> {
    let path = Path::new("src/8.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("ERR: couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("ERR: couldn't read {}: {}", display, why),
        Ok(_) => println!("Succesfully loaded file"),
    };

    let mut raw_parts: Vec<&str> = s.split("\n").collect();
    raw_parts.pop(); //Remove last element. Empty since we split on '\n'

    let mut parts: Vec<Vec<u8>> = Vec::new();

    //Convert to Vec<u8>'s
    let mut tmp_v: Vec<u8> = Vec::new();
    for part in &raw_parts {
        for c in 0..part.len()/2 {
            tmp_v.push(u8::from_str_radix(&part[c*2..c*2+2], 16).unwrap());
        }
        parts.push(tmp_v.clone());
        tmp_v.truncate(0);
    }
    
    parts
}

fn detect(ciphers: &Vec<Vec<u8>>) {

    for cipher in ciphers {
        if detect_single(cipher) == true {
            println!("Found: {:?}", cipher);
        }
    }
    
}

fn detect_single(cipher: &Vec<u8>) -> bool {
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let mut seen = HashSet::new();

    let mut tmp_v: Vec<u8> = Vec::new();
    for c in 0..cipher.len() {
        tmp_v.push(cipher[c]);
        if tmp_v.len() == 16 {blocks.push(tmp_v.clone()); tmp_v.truncate(0);}
    }

    for vect in blocks {
        if !seen.insert(vect) {
            return true;
        }
    }
    false
}