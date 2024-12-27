use std::io;

pub fn pkcs7() {
    println!("--------------------");
    println!("PKCS#7 padding");
    println!("Enter plaintext to be padded:");

    let mut buffer = String::new();
    let mut _plaintext = String::new();
    let mut _blocksize: usize = 0;

    io::stdin()
        .read_line(&mut buffer)
        .expect("ERR: Failed to read plaintext");

    _plaintext = buffer.trim().parse().unwrap();
    buffer = String::new();
    println!("Enter blocksize:");

    io::stdin()
        .read_line(&mut buffer)
        .expect("ERR: Failed to read block size");

    _blocksize = buffer.trim().parse().expect("ERR: blocksize is not a valid size");

    println!("Read: {}", _plaintext);
    println!("Blocksize: {}", _blocksize);

    let blocks = pad_plaintext(&_plaintext, &_blocksize, 4);
    println!("Blocks:\n\t{:?}", blocks);
}

pub fn pad_plaintext(plaintext: &String, blocksize: &usize, padder: u8) ->  Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::new();

    let mut tmp_str: Vec<u8> = Vec::new();
    for c in plaintext.bytes() {
        if tmp_str.len() == *blocksize {
            blocks.push(tmp_str.clone());
            tmp_str.clear();

        }
        tmp_str.push(c);
    }

    if tmp_str.len() != *blocksize || tmp_str.len() != 0 {
        while tmp_str.len() != *blocksize {
            tmp_str.push(padder);
        }
        blocks.push(tmp_str.clone());
    }

    blocks
}

