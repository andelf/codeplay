use std::str;

/// Convert hex to base64
pub fn hex_to_base64(raw: &[u8]) -> Vec<u8> {
    static CODES: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut encoded = Vec::with_capacity(raw.len() / 2 / 3 * 4 + 2);
    for bytes in raw.chunks(6) {
        let mut bits = unsafe { u32::from_str_radix(str::from_utf8_unchecked(bytes), 16).unwrap() };
        match bytes.len() {
            6 => {
                encoded.push(CODES[((bits >> 18) & 0b111111) as usize]);
                encoded.push(CODES[((bits >> 12) & 0b111111) as usize]);
                encoded.push(CODES[((bits >> 6) & 0b111111) as usize]);
                encoded.push(CODES[(bits & 0b111111) as usize]);
            }
            4 => {
                bits <<= 8;
                encoded.push(CODES[((bits >> 18) & 0b111111) as usize]);
                encoded.push(CODES[((bits >> 12) & 0b111111) as usize]);
                encoded.push(CODES[((bits >> 6) & 0b111111) as usize]);
                encoded.push(b'=');
            }
            2 => {
                bits <<= 16;
                encoded.push(CODES[((bits >> 18) & 0b111111) as usize]);
                encoded.push(CODES[((bits >> 12) & 0b111111) as usize]);
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
