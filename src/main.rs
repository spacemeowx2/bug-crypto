use new_crypto::v1::{Cipher, CipherKind::SS_RC4_MD5};
use old_crypto;

const KEY: &'static [u8] = b"1234123412341234";
const EMPTY_IV: &'static [u8] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
const EMPTY: &'static [u8] = b"";
const MSG: &'static [u8] = b"Hello, world!";

fn new_enc() -> Vec<u8> {
    let mut cipher = Cipher::new(SS_RC4_MD5, KEY, EMPTY);

    let mut helloworld = MSG.to_vec();

    cipher.encrypt_packet(&mut helloworld);

    helloworld
}

fn main() {
    let mut helloworld = new_enc();
    let mut cipher = Cipher::new(SS_RC4_MD5, KEY, EMPTY);

    assert!(cipher.decrypt_packet(&mut helloworld));
    assert_eq!(helloworld, MSG);

    let mut helloworld = new_enc();
    let mut cipher =
        old_crypto::v1::Cipher::new(old_crypto::v1::CipherKind::SS_RC4_MD5, KEY, EMPTY_IV);

    assert!(cipher.decrypt_packet(&mut helloworld));
    assert_eq!(helloworld, MSG);

    println!("{:?}", helloworld);
}
