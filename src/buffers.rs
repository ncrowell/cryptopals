pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len());
    let mut out = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        out.push(a[i] ^ b[i]);
    }
    out
}

pub fn xor_with_byte(buf: &[u8], b: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(buf.len());
    for i in 0..buf.len() {
        out.push(buf[i] ^ b);
    }
    out
}

#[allow(unused_imports)]
#[allow(dead_code)]
mod test {
    use encoding::hex::decode;
    use super::*;

    #[test]
    fn matasano_xor_test() {
        let a = decode("1c0111001f010100061a024b53535009181c");
        let b = decode("686974207468652062756c6c277320657965");
        let out = decode("746865206b696420646f6e277420706c6179");

        assert!(xor(&a[], &b[]) == out);
    }

    #[test]
    fn xor_byte_test() {
        let top4bits: [u8; 2] = [240, 240];
        println!("{:?}", xor_with_byte(&top4bits, 31));
        assert!(xor_with_byte(&top4bits, 15) == [255, 255]);
        assert!(xor_with_byte(&top4bits, 31) == [239, 239]);
    }

}

