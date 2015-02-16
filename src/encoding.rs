pub mod hex {
    pub fn decode(hex: &str) -> Vec<u8> {
        assert!(hex.len() % 2 == 0);
        let mut bytes = Vec::with_capacity(hex.len() / 2);

        for pair in hex.as_bytes().chunks(2) {
            bytes.push((dec_char(pair[0]) << 4) | dec_char(pair[1]));
        }

        bytes
    }

    pub fn encode(bytes: &[u8]) -> String {
        let mut enc_bytes = Vec::with_capacity(bytes.len() * 2);

        for b in bytes.iter() {
            enc_bytes.push(enc_bits(*b >> 4));
            enc_bytes.push(enc_bits(*b & 15));
        }

        String::from_utf8(enc_bytes).ok().unwrap()
    }

    pub fn enc_bits(bits: u8) -> u8 {
        assert!(bits < 16);
        if bits < 10 { 48 + bits }
        else { 97 + bits - 10 }
    }

    pub fn dec_char(c: u8) -> u8 {
        if c <= 57 {
            c - 48
        } else if c <= 70 {
            c - 65 + 10
        } else {
            assert!(c < 123);
            c - 97 + 10
        }
    }
}

pub mod b64 {
    static PADDING_CHAR: u8 = b'=';

    pub fn decode(b64s: &str) -> Vec<u8> {
        let b64 = b64s.as_bytes();
        let mut bytes: Vec<u8> = Vec::with_capacity(3 * (b64.len() / 4)); 
        assert!(b64.len() % 4 == 0);
        
        if b64.len() == 0 {
            return bytes
        }

        for quad in b64.chunks(4) {
            let bits24 = 
                  (to_byte(quad[0]) as u32) << 18
                | (to_byte(quad[1]) as u32) << 12
                | (to_byte(quad[2]) as u32) << 6
                | (to_byte(quad[3]) as u32);

            bytes.push(((bits24 >> 16) & 255) as u8);
            bytes.push(((bits24 >> 8)  & 255) as u8);
            bytes.push(( bits24        & 255) as u8);
        }

        if b64[b64.len() - 1] == PADDING_CHAR {
            bytes.pop();
            if b64[b64.len() - 2] == PADDING_CHAR {
                bytes.pop();
            }
        }

        bytes
    }

    pub fn encode(bytes: &[u8]) -> String {
        let len = bytes.len();
        let leftover = len % 3;
        let safe_len = len - leftover;
        let mut encoded: Vec<u8> = Vec::with_capacity(4 * (1 + bytes.len() / 3));

        if len == 0 {
            return String::from_utf8(encoded).ok().unwrap()
        }

        if len >= 3 {
            for trio in bytes[..safe_len].chunks(3) {
                let bits24 = (trio[0] as u32) << 16
                           | (trio[1] as u32) << 8
                           | (trio[2] as u32);
                encoded.push(convert_quad(bits24, 18));
                encoded.push(convert_quad(bits24, 12));
                encoded.push(convert_quad(bits24, 6));
                encoded.push(convert_quad(bits24, 0));
            }
        }

        if leftover > 0 {
            let mut last_trio: u32 = (bytes[safe_len] as u32) << 16;
            if leftover == 2 {
                last_trio |= (bytes[safe_len + 1] as u32) << 8;
            }
            encoded.push(convert_quad(last_trio, 18));
            encoded.push(convert_quad(last_trio, 12));
            encoded.push(if leftover == 2 { convert_quad(last_trio, 6) }
                         else { PADDING_CHAR });
            encoded.push(PADDING_CHAR);
        }

        String::from_utf8(encoded).ok().unwrap()
    }

    #[inline]
    fn convert_quad(i: u32, shift: u8) -> u8 {
        from_byte((i >> shift & 63) as u8)
    }

    #[inline]
    fn to_byte(byte: u8) -> u8 {
        if byte == 43 { 62 }
        else if byte == 47 { 63 }
        else if byte >= 48 && byte < 58  { byte - 48 + 52}
        else if byte >= 65 && byte < 91  { byte - 65}
        else if byte >= 97 && byte < 123 { byte - 97 + 26}
        else if byte == PADDING_CHAR { 0 }
        else { panic!("Invalid byte") } // never happens
    }

    #[inline]
    fn from_byte(bits: u8) -> u8 {
        assert!(bits < 64);
        if bits < 26       { 65 + bits}
        else if bits < 52  { 97 + bits - 26}
        else if bits < 62  { 48 + bits - 52}
        else if bits == 62 { 43 } // ascii "+"
        else if bits == 63 { 47 } // ascii "/"
        else { panic!("Invalid byte") } // never happens
    }

}
#[cfg(test)]
mod test {
    use super::*;

    static HEX_STRING: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    static B64_STRING: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    // BASE64("") = ""
    // BASE64("f") = "Zg=="
    // BASE64("fo") = "Zm8="
    // BASE64("foo") = "Zm9v"
    // BASE64("foob") = "Zm9vYg=="
    // BASE64("fooba") = "Zm9vYmE="
    // BASE64("foobar") = "Zm9vYmFy"
    static RFC_TESTS: &'static [(&'static [u8], &'static str)] = &[
        (b"", ""),
        (b"f", "Zg=="),
        (b"fo", "Zm8="),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg=="),
        (b"fooba", "Zm9vYmE="),
        (b"foobar", "Zm9vYmFy"),
    ];

    #[test]
    pub fn rfc_encode_test() {
        for &(s, encoded) in RFC_TESTS {
            assert!(b64::encode(s) == encoded);
        }
    }

    #[test]
    pub fn rfc_decode_test() {
        for &(s, encoded) in RFC_TESTS {
            assert!(b64::decode(encoded) == s);
        }
    }

    #[test]
    pub fn matasano_hex_to_b64_test() {
        let bytes: Vec<u8> = hex::decode(HEX_STRING);
        let b64: String = b64::encode(&bytes[]);
        assert!(&b64[] == B64_STRING);
    }

    #[test]
    pub fn matasano_b64_to_hex_test() {
        let bytes: Vec<u8> = b64::decode(B64_STRING);
        let hex: String = hex::encode(&bytes);
        assert!(&hex[] == HEX_STRING);
    }

    #[test]
    pub fn wiki_b64_encode_test() {
        let s: &str = "Man";
        let encode = b64::encode(s.as_bytes());
        println!("\n encode is: '{:?}'", encode);
        assert!(encode == "TWFu");
    }

    #[test]
    pub fn wiki_b64_decode_test() {
        let s = "TWFu";
        let decode = b64::decode(s);
        assert!(&decode[] == "Man".as_bytes());
    }
}
