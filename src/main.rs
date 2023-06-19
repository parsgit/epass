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

    Password::init_config();
    let get_password = Password::get_main_password();
    let pass = Password::new(get_password);
    
    // pass.init_save_keys_path();
    pass.main_menu(true)

}
