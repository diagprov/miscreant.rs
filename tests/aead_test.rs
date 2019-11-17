mod aead_vectors;

use self::aead_vectors::AesSivAeadExample;
use miscreant::{Aead, Aes128PmacSivAead, Aes128SivAead, Aes256PmacSivAead, Aes256SivAead};

#[test]
fn aes_siv_aead_examples_encrypt() {
    let examples = AesSivAeadExample::load_all();

    for example in examples {
        let ciphertext = match example.alg.as_ref() {
            "AES-SIV" => match example.key.len() {
                32 => Aes128SivAead::new(&example.key).encrypt(
                    &example.nonce,
                    &example.ad,
                    &example.plaintext,
                ),
                64 => Aes256SivAead::new(&example.key).encrypt(
                    &example.nonce,
                    &example.ad,
                    &example.plaintext,
                ),
                _ => panic!("unexpected key size: {}", example.key.len()),
            },
            "AES-PMAC-SIV" => match example.key.len() {
                32 => Aes128PmacSivAead::new(&example.key).encrypt(
                    &example.nonce,
                    &example.ad,
                    &example.plaintext,
                ),
                64 => Aes256PmacSivAead::new(&example.key).encrypt(
                    &example.nonce,
                    &example.ad,
                    &example.plaintext,
                ),
                _ => panic!("unexpected key size: {}", example.key.len()),
            },
            _ => panic!("unexpected algorithm: {}", example.alg),
        };

        assert_eq!(ciphertext, example.ciphertext);
    }
}

#[test]
fn aes_siv_aead_examples_decrypt() {
    let examples = AesSivAeadExample::load_all();

    for example in examples {
        let plaintext = match example.alg.as_ref() {
            "AES-SIV" => match example.key.len() {
                32 => Aes128SivAead::new(&example.key).decrypt(
                    &example.nonce,
                    &example.ad,
                    &example.ciphertext,
                ),
                64 => Aes256SivAead::new(&example.key).decrypt(
                    &example.nonce,
                    &example.ad,
                    &example.ciphertext,
                ),
                _ => panic!("unexpected key size: {}", example.key.len()),
            },
            "AES-PMAC-SIV" => match example.key.len() {
                32 => Aes128PmacSivAead::new(&example.key).decrypt(
                    &example.nonce,
                    &example.ad,
                    &example.ciphertext,
                ),
                64 => Aes256PmacSivAead::new(&example.key).decrypt(
                    &example.nonce,
                    &example.ad,
                    &example.ciphertext,
                ),
                _ => panic!("unexpected key size: {}", example.key.len()),
            },
            _ => panic!("unexpected algorithm: {}", example.alg),
        }
        .expect("decrypt failure");

        assert_eq!(plaintext, example.plaintext);
    }
}
