// mod action;
// pub mod login;

mod config;
mod password;

use colored::Colorize;
// use action::Action;
use password::Password;

// use aes_gcm::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     Aes256Gcm, Key, Nonce,
// };
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use sha256::digest;

use sha2::{Digest, Sha256};



fn main() {


    // assert_eq!(result[..], hex!("
    //     3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532
    // ")[..]);

    // let plaintext = b"Hello, world!";
    // let hash = digest(plaintext);
    // let key: [u8; 32] = hash.;
    // let key = Aes256Gcm::generate_key(OsRng);

    // Transformed from a byte array:
    // let key: &[u8; 32] = &[42; 32];
    // let key: &[u8; 32] = &text_to_bytes("hello ben");
    // let key: &Key<Aes256Gcm> = key.into();

    // // // Note that you can get byte array from slice using the `TryInto` trait:
    // // let key: &[u8] = &[42; 32];
    // // let key: [u8; 32] = key.try_into()?;

    // // Alternatively, the key can be transformed directly from a byte slice
    // // (panicks on length mismatch):
    // // let key = Key::<Aes256Gcm>::from_slice(&key);

    // let cipher = Aes256Gcm::new(&key);
    // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    // let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
    // let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();

    // println!("{:?}=={:?}",plaintext, ciphertext);
    // assert_eq!(&plaintext, b"plaintext message");

    // if &plaintext== b"plaintext message"{
    //     println!("is Ok : '{}'", std::str::from_utf8(&plaintext).unwrap());

    // }
    // let key: &[u8; 32] = &[42; 32];
    // let key: &Key<Aes256Gcm> = key.into();
    // let ciphertext = Password::decode(val.as_str(), "this is my text").unwrap();
    // let ciphertext = encode(key, plaintext).unwrap();
    // assert_eq!(ciphertext, "bf8e5d3b2bb3fd4e00c58ddafaba2ff3");

    // let decoded_plaintext = decode(key, &ciphertext).unwrap();
    // assert_eq!(decoded_plaintext, plaintext);
    // println!("code: {}", ciphertext);

    // Password::check_current_pass();

    let get_password = Password::get_main_password();
    // let app = Password{password: ""};
    let pass = Password::new(get_password);
    
    pass.init_config();
    pass.init_save_keys_path();
    pass.main_menu(true)
    // Password::init_config();
    // Password::init_save_keys_path();

    // Password::main_menu(true);

}
