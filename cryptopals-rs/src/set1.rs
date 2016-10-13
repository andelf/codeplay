use std::str;

extern crate itertools;
use self::itertools::Itertools;


pub fn hex_to_bytes(encoded: &[u8]) -> Vec<u8> {
    assert_eq!(encoded.len() & 2, 0);
    let mut bytes = Vec::with_capacity(encoded.len() / 2);
    for hex in encoded.chunks(2) {
        let byte = unsafe { u8::from_str_radix(str::from_utf8_unchecked(hex), 16).unwrap() };
        bytes.push(byte);
    }
    bytes
}


pub fn bytes_to_hex(raw: &[u8]) -> Vec<u8> {
    static CODES: &'static [u8] = b"0123456789abcdef";

    let mut encoded = Vec::with_capacity(raw.len() * 2);
    for &byte in raw {
        encoded.push(CODES[(byte >> 4) as usize]);
        encoded.push(CODES[(byte & 0b1111) as usize]);
    }
    encoded
}

/// Convert hex to base64
pub fn hex_to_base64(raw: &[u8]) -> Vec<u8> {
    static CODES: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut encoded = Vec::with_capacity(raw.len() / 2 / 3 * 4 + 2);
    for bytes in raw.chunks(6) {
        let mut bits = unsafe { usize::from_str_radix(str::from_utf8_unchecked(bytes), 16).unwrap() };
        match bytes.len() {
            6 => {
                encoded.push(CODES[(bits >> 18) & 0b111111]);
                encoded.push(CODES[(bits >> 12) & 0b111111]);
                encoded.push(CODES[(bits >> 6) & 0b111111]);
                encoded.push(CODES[bits & 0b111111]);
            }
            4 => {
                bits <<= 8;
                encoded.push(CODES[(bits >> 18) & 0b111111]);
                encoded.push(CODES[(bits >> 12) & 0b111111]);
                encoded.push(CODES[(bits >> 6) & 0b111111]);
                encoded.push(b'=');
            }
            2 => {
                bits <<= 16;
                encoded.push(CODES[(bits >> 18) & 0b111111]);
                encoded.push(CODES[(bits >> 12) & 0b111111]);
                encoded.extend(b"==");
            }
            _ => unreachable!(),
        }
    }
    encoded.shrink_to_fit();
    encoded
}

/// takes two equal-length buffers and produces their XOR combination
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len(), "two equal-length buffers");
    static CODES: &'static [u8] = b"0123456789abcdef";
    let mut xord = Vec::with_capacity(a.len());
    for (hex1, hex2) in a.chunks(2).zip(b.chunks(2)) {
        let a = unsafe { u8::from_str_radix(str::from_utf8_unchecked(hex1), 16).unwrap() };
        let b = unsafe { u8::from_str_radix(str::from_utf8_unchecked(hex2), 16).unwrap() };
        let c = a ^ b;
        xord.push(CODES[(c >> 4) as usize]);
        xord.push(CODES[(c & 0b1111) as usize]);
    }
    xord
}



pub fn single_byte_xor_cipher(encoded: &[u8], cipher: u8) -> Vec<u8> {
    let mut bytes = hex_to_bytes(encoded);
    bytes.iter_mut().foreach(|b| *b ^= cipher);
    bytes
}


pub fn repeating_key_xor(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(text.len());
    for (&byte, &cipher) in text.iter().zip(key.iter().cycle()) {
        ret.push(byte ^ cipher);
    }
    ret
}



#[test]
fn test_repeating_key_xor() {
    let text = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let should_be_encrypted = b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let encrypted = repeating_key_xor(text.as_ref(), b"ICE".as_ref());
    assert_eq!(bytes_to_hex(&encrypted), should_be_encrypted.as_ref());
}



#[test]
fn test_convert_hex_to_base64() {
    let hex_string = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_string = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(&hex_to_base64(hex_string)[..], base64_string.as_ref());
}

#[test]
fn test_fixed_xor() {
    let a = b"1c0111001f010100061a024b53535009181c";
    let b = b"686974207468652062756c6c277320657965";
    let xord = b"746865206b696420646f6e277420706c6179";

    assert_eq!(&fixed_xor(a, b)[..], xord.as_ref());
}


#[test]
fn test_single_byte_xor_cipher() {
    let hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    assert_eq!(&single_byte_xor_cipher(hex, 'X' as u8)[..],
               b"Cooking MC's like a pound of bacon".as_ref());
}



#[cfg(test)]
mod test {
    use test::Bencher;
    use super::*;

    // 332 ns/iter (+/- 11)
    #[bench]
    fn bench_convert_hex_to_base64(b: &mut Bencher) {
        b.iter(|| {
            let hex_string = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
            hex_to_base64(hex_string);
        });
    }

    // 303 ns/iter (+/- 73)
    #[bench]
    fn bench_fixed_xor(b: &mut Bencher) {
        b.iter(|| {
            let a = b"1c0111001f010100061a024b53535009181c";
            let b = b"686974207468652062756c6c277320657965";
            fixed_xor(a, b);
        })
    }
}
