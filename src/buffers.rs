pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len());

    let mut out = Vec::with_capacity(a.len());
    for i in range(0, a.len()) {
        out.push(a[i] ^ b[i]);
    }

    out
}

#[allow(unused_imports)]
mod test {
    use encoding::hex::decode;

    static TEST_A: &'static str = "1c0111001f010100061a024b53535009181c";
    static TEST_B: &'static str = "686974207468652062756c6c277320657965";
    static TEST_OUT: &'static str = "746865206b696420646f6e277420706c6179";

    #[test]
    fn matasano_xor_test() {
        let a = decode(TEST_A);
        let b = decode(TEST_B);
        let out = decode(TEST_OUT);

        assert!(super::xor(a.as_slice(), b.as_slice()) == out);
    }
}

