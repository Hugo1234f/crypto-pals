use std::io;

pub fn sb_xor() {
    println!("--------------------");
    println!("Single-byte XOR cipher");
    println!("Encoded hex string:");

    let mut hex_text = String::new();
    io::stdin()
        .read_line(&mut hex_text)
        .expect("ERR: Could not read input");
    println!("");
    
    println!("Best guess:");
    let plaintext = calc_sb_xor(&hex_text[..]);
    if plaintext.len() == 0 {
        println!("Err");
    }else {
        println!("{}", plaintext);
    }

}
//Calculates a best guess
pub fn calc_sb_xor(hex_text: &str) -> String {
    let _tests: Vec<Vec<u8>> = Vec::new();  
    let mut byte_text: Vec<u8> = Vec::new();

    //Convert hex to bytes
    for c in 0..(hex_text.len()/2) {
        let res = u8::from_str_radix(&hex_text[2*c..2*c+2], 16);
        match res {
            Ok(e) => byte_text.push(e),
            Err(_) => continue,
        };
    }

    //Exclude all combinations containing non-printable characters
    let mut valid_combos: Vec<(Vec<u8>, u8)> = Vec::new();
    for c in 0..255 {
        //let mut valid_string = true;
        let mut tmp_s: Vec<u8> = Vec::new();
        tmp_s.clear();
        for byte_c in &byte_text {
            let xored_c = byte_c ^ c;
            //if xored_c < 32 || xored_c > 126 {valid_string = false;} //Exclude all non-printable characters. 
            tmp_s.push(xored_c);
        }
        
        valid_combos.push((tmp_s, c));
    }

    //Calculate the best guess
    let mut lowest_dev = 999.0;
    let mut lowest_dev_vec: Vec<u8> = Vec::new();
    for combo in valid_combos {
        let new_dev = calculate_deviation(&combo.0);
        if new_dev < lowest_dev {
            lowest_dev = new_dev;
            lowest_dev_vec = combo.0;
        }
    }

    match String::from_utf8(lowest_dev_vec) {
        Ok(guess) => return guess,
        Err(_) => return String::from(""),
    };
}
//Returns a rounded number representing how many instances of 'c' there should be in a string of length 'length' (spaces excluded)
pub fn estimate_char_num(length: usize, c: char) -> f64 {
    let prob_table: Vec<(char, f64)> = vec![('a', 0.082), ('b', 0.015), ('c', 0.028), ('d', 0.043),
                        ('e', 0.127), ('f', 0.022), ('g', 0.02), ('h', 0.061),
                        ('i', 0.07), ('j', 0.0015), ('k', 0.0077), ('l', 0.04),
                        ('m', 0.024), ('n', 0.067), ('o', 0.075), ('p', 0.019),
                        ('q', 0.00095), ('r', 0.06), ('s', 0.063), ('t', 0.091),
                        ('u', 0.028), ('v', 0.0098), ('w', 0.024), ('x', 0.015),
                        ('y', 0.02), ('z', 0.0074), (' ', 0.13)];
    for (key, value) in &prob_table {
        if *key == c {
            return value * (length as f64);
        }
    }
    0.0
}
//Calculate how much a string deeviates from the expected character frequency distribution
pub fn calculate_deviation(ciphertext: &Vec<u8>) -> f64 {
    let mut deviaton: f64 = 0.0;

    for chosen_c in ciphertext {
        let mut count = 0;
        for counter in ciphertext {
            if counter == chosen_c {count += 1;}
        }

        deviaton += ((count as f64) - estimate_char_num(ciphertext.len(), *chosen_c as char)).powf(2.0);
    }

    deviaton / ciphertext.len() as f64
}
pub fn calculate_deviation_u32(ciphertext: &Vec<u32>) -> f64 {
    let mut deviaton: f64 = 0.0;
    
    for chosen_c in ciphertext {
        let mut count = 0;
        for counter in ciphertext {
            if counter == chosen_c {count += 1;}
        }

        deviaton += ((count as f64) - estimate_char_num(ciphertext.len(), (*chosen_c as u8) as char)).powf(2.0);
    }

    deviaton / ciphertext.len() as f64
}