use std::str;

use trit::Trit;
use text::*;

#[test]
fn text_encode_decode() {
    let mut trits = [Trit::Zero; 1024];
    let s1 = "⸘I like to éat 🍎 and 🍌 wheñ it is 100℉ oütside‽";

    let len1 = encode_str(mut_ptr!(trits), s1);
    let (s2, len2) = decode_str(ptr!(trits));

    assert_eq!(len1, len2);
    assert_eq!(s1, &s2[..]);
}
