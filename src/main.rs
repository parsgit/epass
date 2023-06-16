mod action;
pub mod aes;

mod password;

use colored::Colorize;
// use action::Action;
use password::Password;

// use aes_gcm::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     Aes256Gcm, Key, Nonce,
// };
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
use sha256::digest;

use sha2::{Digest, Sha256};



fn main() {
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
    Password::init_config();
    Password::check_current_pass();
    Password::init_save_keys_path();
    Password::main_menu(true);

    // Action::manage_menu(get_result);

    // let chose_menu = Action::get_main_menu_action();

    // if chose_menu == 3 {
    //     let config_menu = Action::get_config_menu_action();

    //     println!("menu: {}", config_menu);
    //     if config_menu == 1{
    //         Action::save_default_config();
    //     }
    // }
    // println!("Hello, world!");

    // let mut pass = Pass();
    // pass.set_pass(String::from("my_password"));
    // pass.show_pass();
    // pub stract Password{}
    // let mut pass2 = Pass{password:String::from("hello")};

    // let action = Action{};

    // let mut action = Action{};

    // action.show_main_menu();

    // pass2.show_pass();
    // Pass::show_pass();
    // Action::main_menu();
}
