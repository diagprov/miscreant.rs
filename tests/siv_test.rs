mod siv_vectors;

use self::siv_vectors::{AesPmacSivExample, AesSivExample};
use miscreant::{Aes128PmacSiv, Aes128Siv, Aes256PmacSiv, Aes256Siv};

#[test]
fn aes_siv_examples_encrypt() {
    let examples = AesSivExample::load_all();

    for example in examples {
        let ciphertext = match example.key.len() {
            32 => Aes128Siv::new(&example.key).encrypt(&example.ad, &example.plaintext),
            64 => Aes256Siv::new(&example.key).encrypt(&example.ad, &example.plaintext),
            _ => panic!("unexpected key size: {}", example.key.len()),
        };

        assert_eq!(ciphertext, example.ciphertext);
    }
}

#[test]
fn aes_siv_examples_decrypt() {
    let examples = AesSivExample::load_all();

    for example in examples {
        let plaintext = match example.key.len() {
            32 => Aes128Siv::new(&example.key).decrypt(&example.ad, &example.ciphertext),
            64 => Aes256Siv::new(&example.key).decrypt(&example.ad, &example.ciphertext),
            _ => panic!("unexpected key size: {}", example.key.len()),
        }
        .expect("decrypt failure");

        assert_eq!(plaintext, example.plaintext);
    }
}

#[test]
fn aes_pmac_siv_examples_encrypt() {
    let examples = AesPmacSivExample::load_all();

    for example in examples {
        let ciphertext = match example.key.len() {
            32 => Aes128PmacSiv::new(&example.key).encrypt(&example.ad, &example.plaintext),
            64 => Aes256PmacSiv::new(&example.key).encrypt(&example.ad, &example.plaintext),
            _ => panic!("unexpected key size: {}", example.key.len()),
        };

        assert_eq!(ciphertext, example.ciphertext);
    }
}

#[test]
fn aes_pmac_siv_examples_decrypt() {
    let examples = AesPmacSivExample::load_all();

    for example in examples {
        let plaintext = match example.key.len() {
            32 => Aes128PmacSiv::new(&example.key).decrypt(&example.ad, &example.ciphertext),
            64 => Aes256PmacSiv::new(&example.key).decrypt(&example.ad, &example.ciphertext),
            _ => panic!("unexpected key size: {}", example.key.len()),
        }
        .expect("decrypt failure");

        assert_eq!(plaintext, example.plaintext);
    }
}
