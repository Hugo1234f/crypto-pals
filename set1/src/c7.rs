use std::path::Path;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::{general_purpose}};
pub use aes::{
    Aes128,
    cipher::{BlockEncrypt}
};
pub use aes::cipher::KeyInit;
use aes::cipher::BlockDecrypt;
use aes::cipher::consts::U16;


pub fn aes_in_ecb_mode() {
    println!("--------------------");
    println!("AES in ECB mode");

    let key = String::from("YELLOW SUBMARINE").bytes().collect();

    let byte_array = get_file_content();
    
    println!("\nPlaintext:");
    solve_aes(&byte_array, key);
}

pub fn get_file_content() -> Vec<u8> {
    let path = Path::new("src/7.txt");
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

    let s = s.trim();
    let trimmed = general_purpose::STANDARD.decode(s).unwrap();
    trimmed
    
}

fn solve_aes(b_arr: &Vec<u8>, key: Vec<u8>) {
    let mut blocks: Vec<Vec<u8>> = Vec::new();

    let mut tmp_block: Vec<u8> = Vec::new();
    for c in b_arr {
        tmp_block.push(*c);
        if tmp_block.len() == 16 {
            blocks.push(tmp_block.clone());
            tmp_block.clear();
        }
    }
    if tmp_block.len() != 0 {blocks.push(tmp_block);}

    let cipher = Aes128::new_from_slice(&key[..]).unwrap();
    for b in blocks {
        print!("{}",solve_block(&b, &cipher));
    }
    println!("");
}

fn solve_block(block: &Vec<u8>, cipher: &aes::Aes128) -> String {
    let mut block_d = aes::cipher::generic_array::GenericArray::<u8,U16>::clone_from_slice(block);
    cipher.decrypt_block(&mut block_d);
    block_d.to_vec();
    let mut raw_string: Vec<u8> = Vec::new();
    for c in block_d {
        if (c > 31 && c < 127) || c == 10 {raw_string.push(c);}
    }

    String::from_utf8(raw_string).unwrap()
}