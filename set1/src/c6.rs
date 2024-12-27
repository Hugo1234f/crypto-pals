use std::path::Path;
use std::fs::File;
use std::io::Read;

use base64::{Engine as _, engine::{general_purpose}};

use crate::c3;
use crate::c5;

pub fn break_rep_key_xor() {
    println!("--------------------");
    println!("Break repeating-key XOR");

    let file_content = get_file_content();
    let crypto = file_content;
    println!("crypto length: {}", crypto.len());
    break_rep_xor(&crypto);
}

//Copied from c4.rs
pub fn get_file_content() -> Vec<u8> {
    let path = Path::new("src/6.txt");
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

fn break_rep_xor(s1: &Vec<u8>) {
    let norm_ham_dists: Vec<(u32, f64)> = gen_norm_ham_dists(s1);

    let mut best_guess: Vec<u8> = Vec::new();
    for combo in norm_ham_dists {
        print!("Best guess, keysize {}: ", combo.0);
        let tmp = test_solution(combo, s1);
        let mut valid = true;
        for c in &tmp {
            if *c < 32 || *c > 126 {valid = false;}
        }
        if valid {
            println!("{}", String::from_utf8(tmp.clone()).unwrap());
            best_guess = tmp;
            break;
        }
        else {println!("NaN");}
    }
    
    let plaintext = c5::rep_xor(&best_guess, s1);

    println!("\nPlaintext:");
    println!("{}", String::from_utf8(plaintext).unwrap());
}

fn test_solution(test_v: (u32, f64), s: &Vec<u8>) -> Vec<u8> {
    let mut raw_blocks: Vec<Vec<u8>> = Vec::new();
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let length = test_v.0 as usize; 

    
    let mut t_vec: Vec<u8> = Vec::new();
    for i in 0..s.len() {
        t_vec.push(s[i]); 
        if t_vec.len() == length {
            raw_blocks.push(t_vec.clone());
            t_vec.truncate(0);
        }
    }
    if t_vec.len() != 0 {raw_blocks.push(t_vec.clone());}
    t_vec.truncate(0);
    

    for i in 0..length {
        blocks.push(Vec::new());
        for raw_block in &raw_blocks {
            if raw_block.len() >= i+1 {
                blocks[i].push(raw_block[i]);
            }
        }
    }
    
    let mut guess: Vec<u8> = Vec::new();
    for block in blocks {
        guess.push(make_best_guess(&block));
    }
    
    guess
}

fn make_best_guess(byte_text: &Vec<u8>) -> u8 {
    let mut combos: Vec<(Vec<u8>,u8)> = Vec::new();
    
    for c in 0..255 {
        //let mut valid_string = true;
        let mut tmp_s: Vec<u8> = Vec::new();
        tmp_s.clear();
        for byte_c in byte_text {
            let xored_c = byte_c ^ c;
            //if xored_c < 32 || xored_c > 126 {valid_string = false;} //Exclude all non-printable characters. 
            tmp_s.push(xored_c);
        }
        
        combos.push((tmp_s, c));
    }
    
    //Calculate the best guess
    let mut lowest_dev = 9999999.0;
    let mut lowest_dev_char: u8 = b'~'; 
    for combo in combos {
        let new_dev = c3::calculate_deviation(&combo.0);
        if new_dev < lowest_dev {
            lowest_dev = new_dev;
            lowest_dev_char = combo.1;
        }
    }

    lowest_dev_char
}


//Generate a Vector of (keylength, hamming distance) touples
fn gen_norm_ham_dists(s1: &Vec<u8>) -> Vec<(u32, f64)> {
    let mut keysize = 40;
    if s1.len() / 4 <= 40 {
        keysize = s1.len() / 4;
    }

    let mut norm_ham_dists: Vec<(u32, f64)> = Vec::new();
    for i in 2..keysize {
        //Normalize values
        let cycle1 = (&s1[0..i]).to_vec();
        let cycle2 = (&s1[i..2*i]).to_vec();
        let cycle3 = (&s1[2*i..3*i]).to_vec();
        let cycle4 = (&s1[3*i..4*i]).to_vec();
        let cycle_dist = calculate_ham_dist(&cycle1, &cycle2) as f64 / i as f64;
        let cycle_dist2 = calculate_ham_dist(&cycle3, &cycle4) as f64 / i as f64;
        norm_ham_dists.push((i as u32, (cycle_dist+cycle_dist2) / 2.0));
    }

    norm_ham_dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut size = 10;
    if norm_ham_dists.len() < size {size = norm_ham_dists.len();}
    norm_ham_dists[0..size].to_vec()
}


//Calculate the hamming distance between two byte arrays
fn calculate_ham_dist(s1: &Vec<u8>, s2: &Vec<u8>) -> u32 {
    let mut dist = 0;
    let length = s1.len();

    for i in 0..length {
        let mut diff = s1[i] ^ s2[i];
        while diff != 0 {
            if diff & 1 != 0 {dist += 1;}
            diff >>= 1;
        }
    }

    dist
}

