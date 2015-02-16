extern crate ascii;

use std::iter::IteratorExt;
use buffers::xor_with_byte;

use self::ascii::AsciiCast;

pub fn printable_from_bytes(chars: &[u8]) -> Option<String> {
    match chars.to_ascii() {
        Ok(cs) => if cs.iter().all(|b| b.is_print()) {
            Some(cs.to_string())
        } else {
            None
        },
        Err(_) => None
    }
}

pub fn printable(chars: &[u8]) -> bool {
    // Coerce the type to acsii, check to_print().
    chars.iter().all(|b: &u8| b.to_ascii().unwrap().is_print())
}

pub fn find_xored(chars: &[u8]) -> Vec<(String, u8)> {
    let mut printable: Vec<(String, u8)> = vec!();
    for mask in 0..255 {
        let masked = xor_with_byte(chars, mask as u8);
        match printable_from_bytes(&masked[]) {
            Some(s) => { printable.push((s, mask)); },
            None => ()
        }
    }
    printable
}

#[allow(unused_imports)]
mod test {
    use encoding::hex::decode;
    use buffers::xor_with_byte;
    use std::str;
    use super::*;

    #[test]
    pub fn can_print_all() {
        assert!(printable(b"hello"));
    }

    #[test]
    pub fn not_all_print() {
        assert!(!printable(&[1, 2, 'h' as u8, 'c' as u8]));
        assert!(!printable(b"\x01\x02hc"));
    }

    #[test]
    fn decode_xor_test() {
        let s = decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let encoded = &s[];
        //let mut countValid = 0;
        //for x in 0..255 {
            //let unmasked = xor_with_byte(encoded, x as u8);
            //let coerce: Result<&str, str::Utf8Error> = str::from_utf8(unmasked.as_slice());
            //match coerce {
                //Ok(s) => { 
                //}
                //_ => ()
            //};
            //println!("{:?} : {:?}", x as u8, coerce);
        //}

        //println!("Count is: {:?}", countValid);

        let result = ("Cooking MC's like a pound of bacon".to_string(), 88u8);
        //print!("Find xored: {:?}", find_xored(encoded));
        assert!(find_xored(encoded)[0] == result);

    }
}
