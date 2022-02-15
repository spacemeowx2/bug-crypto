const KEY: &'static [u8] = b"1234123412341234";
const EMPTY_IV: &'static [u8] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
const EMPTY: &'static [u8] = b"";
const MSG: &'static [u8] = b"Hello, world!";

fn old_enc() -> Vec<u8> {
    let mut cipher =
        old_crypto::v1::Cipher::new(old_crypto::v1::CipherKind::SS_RC4_MD5, KEY, EMPTY_IV);

    let mut helloworld = MSG.to_vec();

    cipher.encrypt_packet(&mut helloworld);

    helloworld
}

fn main() {
    let mut helloworld = old_enc();
    let mut cipher =
        old_crypto::v1::Cipher::new(old_crypto::v1::CipherKind::SS_RC4_MD5, KEY, EMPTY_IV);

    assert!(cipher.decrypt_packet(&mut helloworld));
    assert_eq!(helloworld, MSG);

    let mut helloworld = old_enc();
    let mut cipher =
        new_crypto::v1::Cipher::new(new_crypto::v1::CipherKind::SS_RC4_MD5, KEY, EMPTY);

    assert!(cipher.decrypt_packet(&mut helloworld));
    // assert failed here
    // assert_eq!(helloworld, MSG);

    let mut helloworld = old_enc();
    let mut cipher =
        fix_crypto::v1::Cipher::new(fix_crypto::v1::CipherKind::SS_RC4_MD5, KEY, EMPTY_IV);

    assert!(cipher.decrypt_packet(&mut helloworld));
    assert_eq!(helloworld, MSG);

    println!("{:?}", helloworld);
}
