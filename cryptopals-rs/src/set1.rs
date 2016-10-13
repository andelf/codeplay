use std::str;
use std::collections::HashMap;
use std::sync::{Once, ONCE_INIT};

extern crate itertools;

use self::itertools::Itertools;


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

#[test]
fn test_convert_hex_to_base64() {
    let hex_string = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_string = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(&hex_to_base64(hex_string)[..], base64_string.as_ref());
}


pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len(), "two equal-length buffers");
    static CODES: &'static [u8] = b"0123456789abcdef";
    static mut XOR_CODES: Option<HashMap<(u8,u8),u8>> = None;
    static INIT_CODES: Once = ONCE_INIT;
    unsafe {
        INIT_CODES.call_once(|| {
            let codes = b"0123456789abcdef".iter().enumerate().cartesian_product(
                b"0123456789abcdef".iter().enumerate()
            )
                .map(|((i,&ih),(j,&jh))| {
                    ((ih,jh), CODES[(i ^ j)])
                }).
                collect();
            XOR_CODES = Some(codes);
        });
    }
    let mut xord = Vec::with_capacity(a.len());
    a.iter().zip(b.iter()).foreach(|(&i,&j)| {
        unsafe {
            XOR_CODES.as_ref().map(|codes| {
                codes.get(&(i,j)).map(|&half_byte| xord.push(half_byte))
            });
        }
    });
    // for (hex1, hex2) in a.chunks(2).zip(b.chunks(2)) {
    //     let a = unsafe { u8::from_str_radix(str::from_utf8_unchecked(hex1), 16).unwrap() };
    //     let b = unsafe { u8::from_str_radix(str::from_utf8_unchecked(hex2), 16).unwrap() };
    //     let c = a ^ b;
    //     xord.push(CODES[(c >> 4) as usize]);
    //     xord.push(CODES[(c & 0b1111) as usize]);
    // }
    xord
}


#[test]
fn test_fixed_xor() {
    let a = b"1c0111001f010100061a024b53535009181c";
    let b = b"686974207468652062756c6c277320657965";
    let xord = b"746865206b696420646f6e277420706c6179";

    assert_eq!(&fixed_xor(a, b)[..], xord.as_ref());
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
