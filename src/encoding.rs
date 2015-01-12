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
    pub fn decode(b64s: &str) -> Vec<u8> {
        let b64 = b64s.as_bytes();
        let mut bytes: Vec<u8> = Vec::with_capacity(3 * (b64.len() / 4)); 
        assert!(b64.len() % 4 == 0);

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

        bytes
    }

    pub fn encode(bytes: &[u8]) -> String {
        let mut encoded: Vec<u8> = Vec::with_capacity(4 * (bytes.len() / 3));

        for trio in bytes.chunks(3) {
            let bits24 =
                  (trio[0] as u32) << 16
                | (trio[1] as u32) << 8
                | (trio[2] as u32);

            encoded.push(from_byte((bits24 >> 18 & 63) as u8));
            encoded.push(from_byte((bits24 >> 12 & 63) as u8));
            encoded.push(from_byte((bits24 >> 6 & 63) as u8));
            encoded.push(from_byte((bits24 & 63) as u8));
        }

        String::from_utf8(encoded).ok().unwrap()
    }

    #[inline]
    fn to_byte(byte: u8) -> u8 {
        if byte == 43 { 62 }
        else if byte == 47 { 63 }
        else if byte >= 48 && byte < 58  { byte - 48 + 52}
        else if byte >= 65 && byte < 91  { byte - 65}
        else if byte >= 97 && byte < 123 { byte - 97 + 26}
        else { panic!("Invalid byte") } // never happens
    }

    #[inline]
    fn from_byte(bits: u8) -> u8 {
        if bits < 26       { 65 + bits}
        else if bits < 52  { 97 + bits - 26}
        else if bits < 62  { 48 + bits - 52}
        else if bits == 62 { 43 } // ascii "+"
        else if bits == 63 { 47 } // ascii "/"
        else { panic!("Invalid byte") } // never happens
    }

}

static HEX_STRING: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
static B64_STRING: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

#[test]
fn matasano_hex_to_b64_test() {
    let bytes: Vec<u8> = hex::decode(HEX_STRING);
    print!("Decoded hex: {:?}\n", bytes);
    let b64: String = b64::encode(bytes.as_slice());
    print!("Converted to b64: {:?}\n", b64);
    //let b64s: &str = b64.as_slice()
    assert!(b64.as_slice() == B64_STRING);
}

#[test]
fn matasano_b64_to_hex_test() {
    let bytes: Vec<u8> = b64::decode(B64_STRING);
    let hex: String = hex::encode(&*bytes);
    //let b64s: &str = b64.as_slice()
    assert!(hex.as_slice() == HEX_STRING);
}

#[test]
fn wiki_b64_encode_test() {
    let s: &str = "Man";
    let encode = b64::encode(s.as_bytes());
    print!("Encode is {:?}\n", encode);
    assert!(encode == "TWFu");
}

#[test]
fn wiki_b64_decode_test() {
    let s = "TWFu";
    let decode = b64::decode(s);
    print!("Decode is {:?}\n", decode);
    assert!(decode.as_slice() == "Man".as_bytes());
}
