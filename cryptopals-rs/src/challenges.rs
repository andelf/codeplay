use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use std::ascii::AsciiExt;


#[path = "lib.rs"]
mod cryptopals;

use self::cryptopals::set1::single_byte_xor_cipher;
use self::cryptopals::set1::fixed_xor;





pub fn set1_challenge3() -> io::Result<()> {

    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut plaintexts = Vec::with_capacity(256);
    for cipher in 0...u8::max_value() {
        plaintexts.push((cipher, single_byte_xor_cipher(hex, cipher)));
    }
    // https://en.wikipedia.org/wiki/Letter_frequency
    plaintexts.sort_by_key(|&(ref cipher, ref bytes)| {
        bytes.iter().filter(|c| b" aeiou".contains(c)).count()
    });
    let result = plaintexts.last().unwrap();

    println!("cipher = {}", result.0 as char);
    println!("text = {:?}", str::from_utf8(&result.1));
    Ok(())
}


pub fn set1_challenge4() -> io::Result<()> {
    let mut buf = Vec::new();
    let mut f = try!(File::open("./priv/4.txt"));
    try!(f.read_to_end(&mut buf));

    for (no, line) in buf.split(|&c| c == '\n' as u8).enumerate() {
        let n = fixed_xor(&line, &line);
        for i in 0 ... 255 {
            let text = single_byte_xor_cipher(line, i);

            if text.is_ascii() {
                println!("encrypted => {} {:?}", no, str::from_utf8(line));
                return Ok(())
            }
        }
    }
    Ok(())
}
