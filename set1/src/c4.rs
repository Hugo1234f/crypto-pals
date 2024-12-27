use crate::c3;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn single_character_xor() {
    println!("--------------------");
    println!("Detect single-character XOR");
    println!("Loading file..."); 
    
    let path = Path::new("src/4.txt");
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

    let raw_hex_strings = s.split('\n');
    let hex_strings: Vec<&str> = raw_hex_strings.collect();
    let mut best_guesses: Vec<String> = Vec::new();

    for mut hex_string in hex_strings {
        hex_string = hex_string.trim();
        let guess = String::from(c3::calc_sb_xor(&hex_string[..]).trim());
        if guess.len() != 0 && is_all_good_ascii(&guess) {
            best_guesses.push(guess);
        }
    }

    println!("Valid strings:");
    for s in best_guesses {
        println!("\t{}", s);
    }

}

fn is_all_good_ascii(text: &str) -> bool {
    for c in text.bytes() {
        if c < 32 || c > 126 {return false}
    }
    return true 
}

