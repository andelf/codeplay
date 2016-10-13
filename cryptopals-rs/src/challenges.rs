#[cfg(not(test))]
#[path = "lib.rs"]
mod cryptopals;

pub fn set1_challenge3() {
    use cryptopals::set1::single_byte_xor_cipher;

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
}
